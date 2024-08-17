
use std::env;
use std::fs;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize)]
pub struct FileMetadata {
    file_name: String,
    file_size: u64,
    last_modified: u64,
    file_type: String,
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

// 파일경로를 받아 메타데이터를 반환(파일명/크기/마지막수정일)
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

    // 파일 확장자로부터 파일 타입 추출
    let file_type = Path::new(&file_path)
        .extension() // 확장자를 가져옴
        .and_then(|ext| ext.to_str()) // &OsStr를 &str로 변환
        .unwrap_or("unknown") // 확장자가 없으면 "unknown"으로 설정
        .to_string(); // String으로 변환

    // FileMetadata 구조체로 반환
    let file_metadata = FileMetadata {
        file_name: file_path,
        file_size: metadata.len(),
        last_modified,
        file_type,
    };

    Ok(file_metadata)
}

// 특정 경로의 새 폴더 생성
// ex) D://test 이면 D드라이브 하위에 test폴더 생성
#[tauri::command]
pub fn create_new_folder(path: String) -> Result<(), String> {
    let path = Path::new(&path);

    if path.exists() {
        return Err("Directory already exists".to_string());
    }

    fs::create_dir_all(&path).map_err(|err| err.to_string())?;

    Ok(())
}

