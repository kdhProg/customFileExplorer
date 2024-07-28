
use std::env;
use std::fs;

#[tauri::command]
pub fn get_current_dir() -> Result<String, String> {
  match env::current_dir() {
      Ok(path) => Ok(path.to_string_lossy().into_owned()),
      Err(e) => Err(format!("오류 발생: {}", e)),
  }
}

#[tauri::command]
pub fn list_files_in_directory(path: String) -> Result<Vec<String>, String> {
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            Ok(files)
        }
        Err(e) => Err(e.to_string()),
    }
}
