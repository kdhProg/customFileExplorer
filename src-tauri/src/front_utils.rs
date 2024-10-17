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

#[command]
pub fn paste_files(files: Vec<String>, target_path: String, cut: bool) -> PasteResult {
    for file in files {
        let source = Path::new(&file);
        let mut destination = Path::new(&target_path).join(source.file_name().unwrap());

        // 동일한 경로일 경우 "_copy" 이름 생성
        if source == destination || is_subdirectory(&source, &destination) {
            destination = generate_copy_name(&destination);
        }

        let result = if cut {
            // 잘라내기: 동일한 경로 이동 방지
            if source == destination {
                return PasteResult {
                    success: false,
                    message: format!("Cannot move to the same location: {}", file),
                };
            }
            move_item(&file, &target_path)
        } else {
            // 복사: 파일과 폴더 구분 처리
            if source.is_file() {
                fs::copy(&source, &destination).map(|_| ())
            } else if source.is_dir() {
                copy_recursive(&source, &destination) // 폴더 복사
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported file type",
                ))
            }
        };

        if let Err(e) = result {
            return PasteResult {
                success: false,
                message: format!("Failed to paste {}: {}", file, e),
            };
        }
    }

    PasteResult {
        success: true,
        message: "Files pasted successfully.".to_string(),
    }
}

// 재귀적으로 폴더 복사
fn copy_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?; // 대상 경로에 폴더 생성

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_recursive(&src_path, &dst_path)?; // 재귀적으로 폴더 복사
        } else {
            fs::copy(&src_path, &dst_path)?; // 파일 복사
        }
    }
    Ok(())
}

// 하위 디렉토리인지 확인
fn is_subdirectory(src: &Path, dst: &Path) -> bool {
    let src_abs = src.canonicalize().unwrap_or(src.to_path_buf());
    let dst_abs = dst.canonicalize().unwrap_or(dst.to_path_buf());
    dst_abs.starts_with(&src_abs)
}

// 파일명에 "_copy" 추가
fn generate_copy_name(path: &Path) -> PathBuf {
    let file_name = path.file_stem().unwrap().to_string_lossy();
    let extension = path.extension().map(|ext| ext.to_string_lossy()).unwrap_or_default();

    let new_file_name = if extension.is_empty() {
        format!("{}_copy", file_name)
    } else {
        format!("{}_copy.{}", file_name, extension)
    };

    path.with_file_name(new_file_name)
}

// 파일 또는 폴더 이동 함수
fn move_item(src: &str, dst: &str) -> io::Result<()> {
    let source = Path::new(src);
    let destination = Path::new(dst).join(source.file_name().unwrap());

    if let Err(_) = fs::rename(&source, &destination) {
        fs::copy(&source, &destination).map(|_| ())?;
        fs::remove_file(&source)?;
    }
    Ok(())
}


// 에러 메시지를 담을 구조체
#[derive(serde::Serialize)]
pub struct TrashResult {
    success: bool,
    message: String,
}

#[command]
pub fn move_files_to_trash(paths: Vec<String>) -> TrashResult {
    for path in paths {
        match trash::delete(&path) {
            Ok(_) => continue, // 성공 시 아무 작업도 하지 않음
            Err(err) => {
                // 하나라도 실패하면 에러 메시지 반환
                return TrashResult {
                    success: false,
                    message: format!("Failed to move {} to trash: {}", path, err),
                };
            }
        }
    }

    // 모든 파일을 성공적으로 이동한 경우
    TrashResult {
        success: true,
        message: "All files moved to trash successfully.".to_string(),
    }
}