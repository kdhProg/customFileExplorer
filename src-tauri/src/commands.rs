
use std::env;
use std::fs;
use std::path::Path;
use trash;
use std::process::Command;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize)]
pub struct FileMetadata {
    file_name: String,
    file_size: u64,
    last_modified: u64,
    file_type: String,
}

#[derive(Debug, serde::Serialize)]
pub struct FileItem {
    file_name: String,
    file_path: String,
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

    // 파일 확장자 추출
    let file_type = Path::new(&file_path)
        .extension() // 확장자를 가져옴
        .and_then(|ext| ext.to_str()) // &OsStr를 &str로 변환
        .unwrap_or("unknown") // 확장자가 없으면 unknown
        .to_string();

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


// 파일명 검색
#[tauri::command]
pub fn search_files(directory: String, keyword: String) -> Result<Vec<FileItem>, String> {
    let dir_path = Path::new(&directory);

    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {}", directory));
    }

    let mut result = Vec::new();

    fn search_in_directory(dir: &Path, keyword: &str, result: &mut Vec<FileItem>) -> Result<(), String> {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            // 디렉토리가 있으면 재귀적으로 검색
                            search_in_directory(&path, keyword, result)?;
                        } else if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                            if file_name.contains(keyword) {
                                result.push(FileItem {
                                    file_name: file_name.to_string(),
                                    file_path: path.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }
                Ok(())
            }
            Err(e) => Err(format!("Failed to read directory: {}", e.to_string())),
        }
    }

    // 루트 디렉토리에서 검색 시작
    search_in_directory(dir_path, &keyword, &mut result)?;

    Ok(result)
}


// 파일삭제(휴지통이동)
#[tauri::command]
pub fn move_to_trash(del_path: String) -> Result<(), String> {
    trash::delete(del_path)
        .map_err(|err| format!("Failed to move file to trash: {}", err.to_string()))
}

// 기본 프로그램으로 해당 파일 실행
#[tauri::command]
pub fn open_file_with_default_program(file_path: &str) -> Result<(), String> {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", "", file_path])
            .status()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(file_path)
            .status()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(file_path)
            .status()
    } else {
        return Err("Unsupported operating system".to_string());
    };

    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("Command exited with status: {}", status)),
        Err(err) => Err(format!("Failed to open file: {}", err)),
    }
}