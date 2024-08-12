
use std::env;
use std::fs;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize)]
pub struct FileMetadata {
    file_name: String,
    file_size: u64,
    last_modified: u64,
}

// 현재 디렉토리 반환
#[tauri::command]
pub fn get_current_dir() -> Result<String, String> {
  match env::current_dir() {
      Ok(path) => Ok(path.to_string_lossy().into_owned()),
      Err(e) => Err(format!("오류 발생: {}", e)),
  }
}

// 인자 디렉토리의 파일목록 반환(문자열)
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

#[tauri::command]
pub fn get_file_metadata(file_path: String) -> Result<FileMetadata, String> {
    let metadata = fs::metadata(&file_path).map_err(|err| err.to_string())?;

    // 파일의 최종 수정일을 UNIX 타임스탬프로 변환
    let last_modified = metadata
        .modified()
        .map_err(|err| err.to_string())?
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_secs();

    // FileMetadata 구조체로 반환
    let file_metadata = FileMetadata {
        file_name: file_path,
        file_size: metadata.len(),
        last_modified,
    };

    Ok(file_metadata)
}

