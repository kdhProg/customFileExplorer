use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tauri::command;
use trash;

#[derive(serde::Serialize)]
pub struct PasteResult {
    success: bool,
    message: String,
}

#[derive(Debug)]
pub struct ItemInfo {
    pub name: String,
    pub is_folder: bool,
}

#[command]
pub fn paste_files(files: Vec<String>, target_path: String, cut: bool) -> PasteResult {
    let existing_items = get_existing_items(&target_path);
    let mut generated_names = std::collections::HashSet::new();

    let mut copy_plan = Vec::new();
    for file in &files {
        let source = Path::new(file);
        let mut destination = Path::new(&target_path).join(source.file_name().unwrap());

        if is_nested_path(source, &destination) {
            return PasteResult {
                success: false,
                message: format!(
                    "Cannot copy '{}' into its own subdirectory '{}'.",
                    source.display(),
                    destination.display()
                ),
            };
        }

        while existing_items.iter().any(|item| item.name == destination.file_name().unwrap().to_string_lossy())
            || generated_names.contains(destination.file_name().unwrap().to_string_lossy().as_ref()) {
            destination = generate_unique_copy_name(&destination, &existing_items, source.is_dir());
        }

        generated_names.insert(destination.file_name().unwrap().to_string_lossy().to_string());
        copy_plan.push((source.to_path_buf(), destination));
    }

    for (source, destination) in copy_plan {
        let result = if cut {
            if source == destination {
                return PasteResult {
                    success: false,
                    message: format!("Cannot move to the same location: {}", source.display()),
                };
            }
            move_item(source.to_str().unwrap(), &target_path)
        } else {
            if source.is_file() {
                fs::copy(&source, &destination).map(|_| ())
            } else if source.is_dir() {
                copy_recursive(&source, &destination)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported file type"))
            }
        };

        if let Err(e) = result {
            return PasteResult {
                success: false,
                message: format!("Failed to paste {}: {}", source.display(), e),
            };
        }
    }

    PasteResult {
        success: true,
        message: "Files pasted successfully.".to_string(),
    }
}

fn is_nested_path(source: &Path, destination: &Path) -> bool {
    destination.starts_with(source) && destination != source.parent().unwrap().join(source.file_name().unwrap())
}


fn get_existing_items(target_path: &str) -> Vec<ItemInfo> {
    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(target_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let is_folder = path.is_dir();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                items.push(ItemInfo {
                    name: name.to_string(),
                    is_folder,
                });
            }
        }
    }
    items
}

fn generate_unique_copy_name(
    path: &Path,
    existing_items: &[ItemInfo],
    is_folder: bool,
) -> PathBuf {
    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let extension = path.extension().map(|ext| ext.to_string_lossy()).unwrap_or_default();

    let mut new_name = if extension.is_empty() {
        format!("{}_copy", file_name)
    } else {
        format!("{}_copy.{}", file_name, extension)
    };

    let mut counter = 2;
    while existing_items.iter().any(|item| item.name == new_name && item.is_folder == is_folder) {
        if extension.is_empty() {
            new_name = format!("{}_copy({})", file_name, counter);
        } else {
            new_name = format!("{}_copy({}).{}", file_name, counter, extension);
        }
        counter += 1;
    }

    path.with_file_name(new_name)
}


fn copy_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}


fn is_subdirectory(src: &Path, dst: &Path) -> bool {
    let src_abs = src.canonicalize().unwrap_or(src.to_path_buf());
    let dst_abs = dst.canonicalize().unwrap_or(dst.to_path_buf());
    dst_abs.starts_with(&src_abs)
}


fn move_item(src: &str, dst: &str) -> io::Result<()> {
    let source = Path::new(src);
    let destination = Path::new(dst).join(source.file_name().unwrap());

    if destination.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("A file or folder with the same name already exists: {}", destination.display())
        ));
    }

    if let Err(_) = fs::rename(&source, &destination) {
        fs::copy(&source, &destination).map(|_| ())?;
        fs::remove_file(&source)?;
    }
    Ok(())
}


#[derive(serde::Serialize)]
pub struct TrashResult {
    success: bool,
    message: String,
}

#[command]
pub fn move_files_to_trash(paths: Vec<String>) -> TrashResult {
    for path in paths {
        match trash::delete(&path) {
            Ok(_) => continue,
            Err(err) => {
                return TrashResult {
                    success: false,
                    message: format!("Failed to move {} to trash: {}", path, err),
                };
            }
        }
    }

    TrashResult {
        success: true,
        message: "All files moved to trash successfully.".to_string(),
    }
}