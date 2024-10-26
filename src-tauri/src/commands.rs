
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use trash;
use std::process::Command;
use serde::{Serialize, Deserialize};

use std::time::{SystemTime, UNIX_EPOCH};

use std::future::Future;
use std::pin::Pin;
use std::fs::File;
use std::io::Write;

use tokio::fs as async_fs;
use tokio::task;

use sysinfo::Disks;

use regex::Regex;


// struct for Drive Infos
#[derive(serde::Serialize)]
pub struct DriveInfo {
    name: String,
    mount_point: String,
    total_space: f64,
    available_space: f64,
}



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





/// 현재 디렉토리 경로 반환
///
///현재 디렉토리 경로를 문자열로 반환합니다.
///
/// # Arguments
///
/// * 
///
/// # Returns
///
/// * `Result<String, String>` - 디렉토리경로
#[tauri::command]
pub fn get_current_dir() -> Result<String, String> {
  match env::current_dir() {
      Ok(path) => Ok(path.to_string_lossy().into_owned()),
      Err(e) => Err(format!("오류 발생: {}", e)),
  }
}

/// 파일리스트 탐색
///
/// 폴더 경로를 받아 해당 경로의 모든 폴더와 파일 리스트 반환
///
/// # Arguments
///
/// * `path` - 디렉토리
///
/// # Returns
///
/// * `Result<Vec<String>, String>` - 디렉토리경로 문자열
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

/// 파일 메타데이터
///
/// 파일의 메타데이터(최종 수정일 / 확장자 / 크기) 반환
///
/// # Arguments
///
/// * `file_path` - 파일
///
/// # Returns
///
/// * `Result<FileMetadata, String>` - 디렉토리경로 문자열
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

/// 새 폴더 생성
///
/// 주어진 경로에 새 폴더를 생성
///
/// # Arguments
///
/// * `path` - 생성할 위치와 폴더명
///
/// # Returns
///
/// * `Result<(), String>` - 실패 시 에러메시지 반환
#[tauri::command]
pub fn create_new_folder(path: String) -> Result<(), String> {
    let path = Path::new(&path);

    if path.exists() {
        return Err("Directory already exists".to_string());
    }

    fs::create_dir_all(&path).map_err(|err| err.to_string())?;

    Ok(())
}


/// Deprecated 
/// 파일 검색 
///
/// 주어진 경로에서 특정 문자열을 포함하는 파일을 탐색
/// tokio를 이용한 비동기처리, 병렬처리 적용
///
/// # Arguments
///
/// * `directory` - 탐색시작 폴더 경로
/// * `keyword` - 포함할 문자열
///
/// # Returns
///
/// * `Result<Vec<FileItem>, String>` - FileItem을 각 원소로 하는 배열 반환
#[tauri::command]
pub async fn search_files_legacy(directory: String, keyword: String) -> Result<Vec<FileItem>, String> {
    let dir_path = PathBuf::from(directory);

    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {:?}", dir_path));
    }

    let mut result = Vec::new();
    search_in_directory(dir_path, keyword, &mut result).await?;
    Ok(result)
}

fn search_in_directory<'a>(
    dir: PathBuf,
    keyword: String,
    result: &'a mut Vec<FileItem>,
) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
    Box::pin(async move {
        let mut handles = Vec::new();

        match async_fs::read_dir(&dir).await {
            Ok(mut entries) => {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    let keyword = keyword.clone();
                    if path.is_dir() {
                        // 디렉토리를 비동기적으로 탐색
                        let handle = tokio::spawn(async move {
                            let mut sub_result = Vec::new();
                            if let Err(e) = search_in_directory(path, keyword, &mut sub_result).await {
                                // 권한 문제나 다른 문제로 인해 탐색할 수 없는 폴더가 있을 경우 오류를 무시하고 넘어감
                                if e.contains("Access is denied") || e.contains("Permission denied") {
                                    println!("Skipping directory due to access error: {}", e);
                                } else {
                                    return Err(e);
                                }
                            }
                            Ok::<_, String>(sub_result)
                        });
                        handles.push(handle);
                    } else {
                        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                            if file_name.contains(&keyword) {
                                result.push(FileItem {
                                    file_name: file_name.to_string(),
                                    file_path: path.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }

                for handle in handles {
                    match handle.await {
                        Ok(Ok(mut sub_result)) => {
                            result.append(&mut sub_result);
                        }
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(format!("Task failed: {:?}", e)),
                    }
                }

                Ok(())
            }
            Err(e) => {
                // 디렉토리를 읽을 수 없는 경우, 오류 무시
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    println!("Skipping directory due to access error: {}", e);
                    Ok(())
                } else {
                    Err(format!("Failed to read directory: {}", e.to_string()))
                }
            }
        }
    })
}


/// 파일 삭제
///
/// 영구삭제가 아닌 휴지통 이동
///
/// # Arguments
///
/// * `del_path` - 휴지통으로 이동할 파일 경로
///
/// # Returns
///
/// * `Result<(), String>` - 실패 시 에러메시지 반환
#[tauri::command]
pub fn move_to_trash(del_path: String) -> Result<(), String> {
    trash::delete(del_path)
        .map_err(|err| format!("Failed to move file to trash: {}", err.to_string()))
}


/// 폴더 여부 확인
///
/// 주어진 경로의 폴더 여부를 확인하여 boolean 반환
///
/// # Arguments
///
/// * `path` - 폴더 여부를 검사할 파일 또는 폴더 경로
///
/// # Returns
///
/// * `Result<bool, String> - true/false 값
#[tauri::command]
pub fn is_directory(path: String) -> Result<bool, String> {
    let metadata = fs::metadata(&path);
    
    match metadata {
        Ok(meta) => Ok(meta.is_dir()),
        Err(err) => Err(err.to_string()),
    }
}



/// 기본 프로그램 실행 
///
/// # Arguments
///
/// * `file_path` - 실행할 대상 파일 경로
///
/// # Returns
///
/// * `Result<(), String>` - 실패 시 에러메시지 반환
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


/// 드라이브 목록 반환
///
/// # Returns
///
/// * `Vec<DriveInfo>`
#[tauri::command]
pub fn get_drive_info() -> Vec<DriveInfo> {
    let disks = Disks::new_with_refreshed_list();
    
    disks.iter().map(|disk| {
        DriveInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space: disk.total_space() as f64 / 1_000_000_000.0, // GB 단위
            available_space: disk.available_space() as f64 / 1_000_000_000.0, // GB 단위
        }
    }).collect()
}


/// Return properties/util_buttons.json
///
/// # Returns
///
/// * `Result<String, String>`
#[tauri::command]
pub fn read_json_file() -> Result<String, String> {

    let file_path = "../src/properties/util_buttons.json";
    match fs::read_to_string(file_path) {
        
        Ok(contents) => {Ok(contents)},
        Err(err) => Err(format!("Error has been occurred during reading files : {}", err)),
    }
}


#[derive(Serialize, Deserialize)]
struct Buttons {
    buttons: Vec<String>,
}

/// Apply util buttons to properties/util_buttons.json
///
/// # Returns
///
/// * 
#[tauri::command]
pub fn save_util_buttons(buttons: Vec<String>) -> Result<(), String> {
    let buttons_data = Buttons { buttons };

    let mut path = PathBuf::from("../src/properties/util_buttons.json");

    let json_data = serde_json::to_string_pretty(&buttons_data)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    let mut file = File::create(&path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(json_data.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}


/// Check existence of input directory or file Path
///
/// # Returns
///
/// bool
#[tauri::command]
pub fn path_exists(dir_path: String) -> bool {
    Path::new(&dir_path).exists()
}



#[tauri::command]
pub fn create_new_item(is_folder: bool, base_path: String) -> Result<String, String> {
    let base_path = Path::new(&base_path);

    // 베이스 경로가 존재하지 않으면 에러 반환
    if !base_path.exists() {
        return Err("The specified base path does not exist.".to_string());
    }

    // 고유한 이름 생성
    let item_name = generate_unique_name(base_path, "new", is_folder)?;
    let full_path = if is_folder {
        base_path.join(&item_name)
    } else {
        // 파일인 경우 .txt 확장자 추가
        base_path.join(format!("{}.txt", item_name))
    };

    // 폴더 생성
    if is_folder {
        fs::create_dir_all(&full_path).map_err(|err| format!("Failed to create folder: {}", err))?;
    } else {
        // 파일 생성 및 내용 작성
        File::create(&full_path)
            .map_err(|err| format!("Failed to create file: {}", err))?
            .write_all(b"New file created")
            .map_err(|err| format!("Failed to write to file: {}", err))?;
    }

    Ok(full_path.to_string_lossy().to_string())
}

/// 고유한 이름을 생성하는 함수 (중복 이름 검사 및 넘버링 처리)
fn generate_unique_name(base_path: &Path, base_name: &str, is_folder: bool) -> Result<String, String> {
    let existing_items = list_files_in_directory(base_path.to_string_lossy().to_string())?;

    // 정규 표현식을 통해 "new", "new(1)", "new(2)" 등의 이름을 탐지
    let regex = Regex::new(r"^new(?:\((\d+)\))?(?:\.txt)?$").unwrap();  // 파일 확장자 고려

    let mut max_number = 0;

    // 모든 항목에서 중복된 이름을 찾음
    for item in existing_items {
        let item_path = Path::new(&item);
        let is_item_folder = item_path.is_dir();

        // 파일과 폴더를 구분해서 필터링
        if is_folder != is_item_folder {
            continue;
        }

        // 파일명 추출 및 확장자 제거 후 정규 표현식 매칭
        if let Some(file_name) = item_path.file_name().and_then(|n| n.to_str()) {
            let name_without_extension = if is_folder {
                file_name.to_string()
            } else {
                // 확장자 제거 (예: "new(1).txt" -> "new(1)")
                file_name.trim_end_matches(".txt").to_string()
            };

            if let Some(captures) = regex.captures(&name_without_extension) {
                if let Some(number) = captures.get(1) {
                    let number = number.as_str().parse::<u32>().unwrap_or(0);
                    max_number = max_number.max(number);
                } else {
                    max_number = max_number.max(1);  // 기본 이름인 "new"가 있는 경우
                }
            }
        }
    }

    // 중복된 이름이 없으면 기본 이름 사용, 아니면 "new(n)" 형식 생성
    let new_name = if max_number == 0 {
        base_name.to_string()
    } else {
        format!("{}({})", base_name, max_number + 1)
    };

    Ok(new_name)
}


#[tauri::command]
pub fn rename_file_or_directory(old_path: String, new_path: String) -> Result<(), String> {
    let old_path = Path::new(&old_path);
    let new_path = Path::new(&new_path);

    // 경로 유효성 검사
    if !old_path.exists() {
        return Err("Invalid path : ".to_string());
    }

    if new_path.exists() {
        return Err("Same name already exists : ".to_string());
    }

    // 파일 또는 디렉토리 이름 변경
    fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;

    Ok(())
}


