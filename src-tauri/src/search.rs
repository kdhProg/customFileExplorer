
// 2024 10 11 20 09

use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;
use std::sync::{Arc};
use tokio::sync::{Mutex, mpsc::{self, Sender}};
use serde::Deserialize;
use std::future::Future;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, NaiveDateTime};
use mime_guess::from_path;
use serde::{Serialize};
use tauri::State;
use tauri::Window;
use num_cpus;
use std::collections::HashSet;  // 중복된 파일 전송 방지용
use std::collections::HashMap;
use std::process::Command;
use std::env;
use windows::Win32::UI::Shell::IsUserAnAdmin;
use std::thread::ThreadId;


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct FileItem {
    pub file_name: String,
    pub file_path: String,
}

#[derive(Deserialize, Clone)]
pub struct SearchOptions {
    #[serde(rename = "customThreadPoolUse")]
    custom_thread_pool_use: bool,
    #[serde(rename = "threadPoolNum")]
    thread_pool_num: String,
    #[serde(rename = "searchScope")]
    search_scope: String,
    #[serde(rename = "customFileContUse")]
    custom_file_cont_use: bool,
    #[serde(rename = "customPropertyUse")]
    custom_property_use: bool,
    #[serde(rename = "customFileSizeUse")]
    custom_file_size_use: bool,
    #[serde(rename = "sizeMax")]
    size_max: u64,
    #[serde(rename = "sizeMin")]
    size_min: u64,
    #[serde(rename = "customFileCrtDateUse")]
    custom_file_crt_date_use: bool,
    #[serde(rename = "crtStart")]
    crt_start: String,
    #[serde(rename = "crtEnd")]
    crt_end: String,
    #[serde(rename = "customFileModiDateUse")]
    custom_file_modi_date_use: bool,
    #[serde(rename = "modiStart")]
    modi_start: String,
    #[serde(rename = "modiEnd")]
    modi_end: String,
    #[serde(rename = "customFileOwnerUse")]
    custom_file_owner_use: bool,
    #[serde(rename = "ownerName")]
    owner_name: String,
    #[serde(rename = "customFileTypeUse")]
    custom_file_type_use: bool,
    #[serde(rename = "fileTypeList")]
    file_type_list: String,
    #[serde(rename = "customSymbolicChk")]
    custom_symbolic_chk: bool,
    #[serde(rename = "customSchMethod")]
    custom_sch_method: String,
}

#[derive(Clone)]
pub struct SearchProcess {
    is_cancelled: Arc<Mutex<bool>>,
    is_completed: Arc<Mutex<bool>>,  // 탐색 완료 여부 추가
    id: String,
}

impl SearchProcess {
    pub fn new() -> Self {
        SearchProcess {
            is_cancelled: Arc::new(Mutex::new(false)),
            is_completed: Arc::new(Mutex::new(false)),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub async fn cancel(&self) {
        // 이미 완료된 작업이라면 취소를 무시
        if self.is_completed().await {
            println!("Search is already completed, cannot cancel.");
            return;
        }
        let mut cancelled = self.is_cancelled.lock().await;
        *cancelled = true;
        println!("Search has been cancelled.");
    }

    pub async fn is_completed(&self) -> bool {
        *self.is_completed.lock().await
    }

    pub async fn mark_as_completed(&self) {
        let mut completed = self.is_completed.lock().await;
        *completed = true;
    }

    pub async fn is_cancelled(&self) -> bool {
        *self.is_cancelled.lock().await
    }

    pub async fn get_info(&self) -> SearchProcessInfo {
        let is_cancelled = *self.is_cancelled.lock().await;  // await로 잠금 해제하고 값을 얻음
        SearchProcessInfo {
            id: self.id.clone(),
            is_cancelled,  // 잠금 해제 후 값을 복사
        }
    }
}



#[derive(Serialize,Debug,Clone)] // 직렬화 가능하게 설정
pub struct SearchProcessInfo {
    id: String,
    is_cancelled: bool,
}

impl SearchProcessInfo {
    pub fn new(id: String) -> Self {
        Self {
            id,
            is_cancelled: false,
        }
    }
}



pub struct AppState {
    search_processes: Mutex<HashMap<String, Arc<SearchProcess>>>,  // 검색 프로세스 저장
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            search_processes: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_process(&self, process_id: String, process: Arc<SearchProcess>) {
        let mut processes = self.search_processes.lock().await;
        processes.insert(process_id, process);
    }

    pub async fn get_process(&self, process_id: &str) -> Option<Arc<SearchProcess>> {
        let processes = self.search_processes.lock().await;
        processes.get(process_id).cloned()
    }

    pub async fn remove_process(&self, process_id: &str) {
        let mut processes = self.search_processes.lock().await;
        processes.remove(process_id);
    }
}

// For Check ThreadPool
async fn track_and_print_thread_ids(thread_ids: Arc<Mutex<HashSet<ThreadId>>>) {
    let ids_lock = thread_ids.lock().await;
    println!("Unique thread IDs used: {:?}", ids_lock);
    println!("Total number of unique threads: {}", ids_lock.len());
}



#[tauri::command]
pub async fn cancel_search(
    process_id: String,
    state: State<'_, AppState>,  // 여러 검색 작업 중에서 특정 프로세스 취소
) -> Result<(), String> {
    if let Some(process) = state.get_process(&process_id).await {
        process.cancel().await;
        println!("Process {} cancelled.", process_id);
    } else {
        println!("Process not found: {}", process_id);
        return Err("Process not found".into());
    }
    Ok(())
}




#[tauri::command]
pub async fn search_files<'a>(
    window: Window,
    keyword: String,
    directory: String,
    options: SearchOptions,
    state: State<'_, AppState>,
) -> Result<SearchProcessInfo, String> {
    let dir_path = PathBuf::from(directory);
    
    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {:?}", dir_path));
    }

    if options.custom_thread_pool_use {
        rayon::ThreadPoolBuilder::new()
            .num_threads(options.thread_pool_num.parse::<usize>().unwrap_or_else(|_| num_cpus::get()))  // 시스템 CPU 코어 수로 설정
            .build_global()
            .unwrap_or_else(|_| println!("Failed to set custom thread pool, using default."));
    }


    let process = Arc::new(SearchProcess::new());
    let process_info = process.get_info().await;
    let process_id = process_info.id.clone();
    println!("Search process created with ID: {}", process.id);  // 프로세스 생성 확인

    // 새로운 SearchProcess를 AppState에 추가
    state.add_process(process_id.clone(), Arc::clone(&process)).await;

    window.emit("process-info", process_info.clone()).expect("Failed to emit process info");
    println!("Backend process ID: {:?}", process.id);

    let result = Arc::new(Mutex::new(Vec::new()));
    let sent_files = Arc::new(Mutex::new(HashSet::new()));
    let (tx, mut rx) = mpsc::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    // For Check ThreadPool
    let thread_ids = Arc::new(Mutex::new(HashSet::<ThreadId>::new())); // 스레드 ID 추적용

    let process_clone = Arc::clone(&process);
    let result_clone = Arc::clone(&result);
    let tx_clone = Arc::clone(&tx);
    
    // For Check ThreadPool
    let thread_ids_clone = Arc::clone(&thread_ids);

    tokio::spawn(async move {
        let process_clone_for_cancel = Arc::clone(&process_clone);
        
        tokio::select! {
            _ = search_in_directory(dir_path, keyword, result_clone, process_clone, options, tx_clone, thread_ids_clone) => {
                println!("Search completed");
            }
            _ = async {
                // 주기적으로 취소 상태를 확인하고 true가 되면 취소
                loop {
                    if process_clone_for_cancel.is_cancelled().await {
                        println!("Search cancelled via select!");
                        break;
                    }
                    // 짧은 대기 시간 후 다시 확인
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            } => {
                println!("Cancel loop executed!");
            }
        }
        drop(tx);  // 채널 닫기
    });
    
    
    

    // 파일 수신 및 전송
    while let Some(file_item) = rx.recv().await {
        let mut sent_files_lock = sent_files.lock().await;
        if sent_files_lock.contains(&file_item.file_path) {
            continue;
        }
        sent_files_lock.insert(file_item.file_path.clone());
        if let Err(e) = window.emit("search-result", file_item.clone()) {
            println!("Failed to emit search result: {:?}", e);
        }
        let mut result_lock = result.lock().await;
        result_lock.push(file_item);
    }

    // 탐색 완료 후 스레드 ID 출력
    track_and_print_thread_ids(Arc::clone(&thread_ids)).await;

    // 탐색이 완료되면 프로세스 완료 처리
    process.mark_as_completed().await;
    
    // 백엔드 로그 확인
    let process_info = process.get_info().await;
    println!("Returning process info: {:?}", process_info);

    // 검색 완료 후 AppState에서 삭제
    state.remove_process(&process_id).await;

    Ok(process_info)  // processInfo 반환
}




fn request_admin_privileges() -> bool {
    // 현재 실행 파일의 경로를 가져옴
    let executable_path = env::current_exe().expect("Failed to get current executable path.");

    // 관리자 권한으로 다시 실행하기 위한 PowerShell 명령어
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process -FilePath \"{}\" -Verb runAs",
            executable_path.display()
        ))
        .output()
        .expect("Failed to execute PowerShell command");

    // 출력 검사 - 성공 여부 확인
    if output.status.success() {
        println!("Admin privileges requested successfully");
        std::process::exit(0); // 프로세스를 종료하고 관리자 권한으로 재시작
    } else {
        eprintln!("Failed to request admin privileges");
        return false;
    }
}

#[cfg(windows)]
fn can_perform_owner_based_search() -> bool {

    // 개발 모드에서는 권한 요청을 건너뛴다.
    if cfg!(debug_assertions) {
        println!("Running in development mode, skipping admin check.");
        return true; // 개발 모드에서는 권한을 체크하지 않고 통과
    }

    println!("Checking if user is admin");

    unsafe {
        // 관리자 권한 여부 확인
        if IsUserAnAdmin().as_bool() {
            return true; // 이미 관리자 권한이 있음
        } else {
            // 관리자 권한이 없으면 요청
            println!("User is not admin, requesting admin privileges via UAC...");
            return request_admin_privileges(); // 관리자 권한 요청 후 결과 반환
        }
    }
}


#[cfg(unix)]
fn can_perform_owner_based_search() -> bool {
    println!("Checking if user is root");
    unsafe { libc::geteuid() == 0 }
}

use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
struct FileOwnerInfo {
    Owner: String,
}

#[cfg(windows)]
fn get_file_owner(path: &std::path::Path) -> Option<String> {
    use std::process::Command;

    // 파일 경로를 얻고 파워쉘 명령어로 파일 소유자 정보를 요청
    let path_str = path.to_string_lossy();
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("(Get-Acl \"{}\" | Select-Object -ExpandProperty Owner)", path_str))
        .output()
        .expect("Failed to execute PowerShell command");

    // 파워쉘 명령이 성공적으로 실행되었는지 확인
    if output.status.success() {
        // 결과를 UTF-8 문자열로 변환
        let owner = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // 소유자 정보가 비어있지 않으면 반환
        if let Some(actual_owner) = owner.split('\\').last() {
            if !actual_owner.is_empty() {
                println!("Owner found: {}", actual_owner);
                return Some(actual_owner.to_string());
            }
        }
    }

    // 실패 시 None 반환
    println!("Failed to get file owner");
    None
}




fn parse_date_to_rfc3339(date_str: &str) -> Result<DateTime<Utc>, String> {
    let rfc3339_str = format!("{}T00:00:00+00:00", date_str);
    DateTime::parse_from_rfc3339(&rfc3339_str)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Failed to parse date: {}", e))
}

fn should_filter_file_by_metadata(path: &Path, options: &SearchOptions) -> bool {
    println!("Filtering file by metadata: {:?}", path);
    if let Ok(metadata) = fs::metadata(path) {
        // 파일 크기 필터링 - 파일인 경우에만 적용
        if options.custom_file_size_use && metadata.is_file() {
            let file_size = metadata.len();
            if file_size > options.size_max || file_size < options.size_min {
                println!("File filtered by size: {}", file_size);
                return true;
            }
        }

        // 생성일 필터링 - 폴더와 파일 모두에 적용 가능
        if options.custom_file_crt_date_use {
            if let Ok(created) = metadata.created() {
                let crt_date = DateTime::<Utc>::from(created);
                
                if let Ok(start_time) = parse_date_to_rfc3339(&options.crt_start) {
                    if crt_date < start_time {
                        println!("File or folder filtered by creation date: {:?}", crt_date);
                        return true;
                    }
                }
                
                if let Ok(end_time) = parse_date_to_rfc3339(&options.crt_end) {
                    if crt_date > end_time {
                        println!("File or folder filtered by end date: {:?}", crt_date);
                        return true;
                    }
                }
            }
        }

        // 수정일 필터링 - 폴더와 파일 모두에 적용 가능
        if options.custom_file_modi_date_use {
            if let Ok(modified) = metadata.modified() {
                let modi_date = DateTime::<Utc>::from(modified);
                
                if let Ok(start_time) = parse_date_to_rfc3339(&options.modi_start) {
                    if modi_date < start_time {
                        println!("File or folder filtered by modified date: {:?}", modi_date);
                        return true;
                    }
                }
                
                if let Ok(end_time) = parse_date_to_rfc3339(&options.modi_end) {
                    if modi_date > end_time {
                        println!("File or folder filtered by end modified date: {:?}", modi_date);
                        return true;
                    }
                }
            }
        }
        

        

        if options.custom_file_owner_use {
            // 소유자 필터링 - 폴더와 파일 모두에 적용 가능
            if !can_perform_owner_based_search() {
                println!("Insufficient permissions to perform owner-based search.");
                return true;
            }

            let owner_name = options.owner_name.to_lowercase();
            #[cfg(windows)]
            {
                if let Some(actual_owner_name) = get_file_owner(path) {
                    if !actual_owner_name.to_lowercase().contains(&owner_name) {
                        println!("File or folder filtered by owner: {}", actual_owner_name);
                        return true;
                    }
                } else {
                    println!("Failed to retrieve owner information. Skipping file or folder.");
                    return true;  // 소유자 정보를 가져오지 못한 경우 필터링 처리
                }
            }

            #[cfg(unix)]
            {
                use nix::unistd::Uid;
                let owner_uid = metadata.uid();
                let actual_owner_name = Uid::from_raw(owner_uid).to_string();
                if !actual_owner_name.to_lowercase().contains(&owner_name) {
                    println!("File or folder filtered by owner: {}", actual_owner_name);
                    return true;
                }
            }
        }

        // 파일 확장자 필터링 - 파일인 경우에만 적용
        if options.custom_file_type_use && metadata.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let allowed_extensions: Vec<&str> = options.file_type_list.split_whitespace().collect();
                if !allowed_extensions.iter().any(|&allowed_ext| ext == allowed_ext.trim_start_matches('.')) {
                    println!("File filtered by extension: {}", ext);
                    return true;
                }
            }
        }
    }

    false
}


fn is_text_file(path: &Path) -> bool {
    println!("Checking if file is text: {:?}", path);
    if let Some(mime) = from_path(path).first() {
        return mime.type_() == "text";
    }
    false
}

fn search_in_directory<'a>(
    dir: PathBuf,
    keyword: String,
    result: Arc<Mutex<Vec<FileItem>>>,
    process: Arc<SearchProcess>,
    options: SearchOptions,
    tx: Arc<Mutex<Sender<FileItem>>>,
    thread_ids: Arc<Mutex<HashSet<ThreadId>>>, // For Check ThreadPool
) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
    Box::pin(async move {
        let mut handles = Vec::new();

        match async_fs::read_dir(&dir).await {
            Ok(mut entries) => {
                while let Ok(Some(entry)) = entries.next_entry().await {

                    tokio::task::yield_now().await;

                    // 현재 스레드의 ID를 가져와 추적
                    let thread_id = std::thread::current().id();
                    let mut thread_ids_lock = thread_ids.lock().await;
                    thread_ids_lock.insert(thread_id);

                    let path = entry.path();
                    let keyword = keyword.clone();

                    if process.is_cancelled().await {
                        println!("Search cancelled during directory scan.");
                        return Ok(());
                    }

                    let metadata = match fs::metadata(&path) {
                        Ok(meta) => meta,
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                println!("Skipping file due to access error: {}", e);
                                continue;
                            } else {
                                return Err(format!("Error reading metadata: {}", e));
                            }
                        }
                    };

                    if path.is_symlink() && !options.custom_symbolic_chk {
                        continue;
                    }

                    // Metadata ( file Property chk)
                    if options.custom_property_use {
                        if should_filter_file_by_metadata(&path, &options) {
                            println!("File filtered by metadata: {:?}", path);
                            continue;  // 메타데이터 필터링 적용
                        }
                    }

                    // 각 검색 방식에 따라 바로 분기
                    match options.custom_sch_method.as_str() {
                        "1" => {
                            search_with_regex(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                        "2" => {
                            search_with_fuzzy(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                        "3" => {
                            search_with_index(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                        _ => {
                            search_default(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                    }

                    // 폴더일 경우 내부 폴더도 재귀적으로 검색
                    if metadata.is_dir() {
                        let handle = tokio::spawn({
                            let process = Arc::clone(&process);
                            let tx = Arc::clone(&tx);
                            let result_clone = Arc::clone(&result);
                            let options_clone = options.clone();

                            // For Check ThreadPool
                            let thread_ids_clone = Arc::clone(&thread_ids);

                            async move {
                                let sub_result = Vec::new();
                                if let Err(e) = search_in_directory(path, keyword, result_clone, process, options_clone, tx, thread_ids_clone).await {
                                    if e.contains("Access is denied") || e.contains("Permission denied") {
                                        println!("Skipping directory due to access error: {}", e);
                                    } else {
                                        return Err(e);
                                    }
                                }
                                Ok::<_, String>(sub_result)
                            }
                        });
                        handles.push(handle);
                    }
                }

                for handle in handles {
                    match handle.await {
                        Ok(Ok(mut sub_result)) => {
                            let mut result_lock = result.lock().await;
                            result_lock.append(&mut sub_result);
                        }
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(format!("Task failed: {:?}", e)),
                    }
                }

                Ok(())
            }
            Err(e) => {
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


async fn search_default(
    path: &Path,
    keyword: &str,
    options: &SearchOptions,
    metadata: &fs::Metadata,
    tx: &Arc<Mutex<Sender<FileItem>>>,
) -> Result<(), String> {
    let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();

    // 탐색 스코프에 따른 분기 처리
    if options.search_scope == "1" && metadata.is_dir() {
        return Ok(()); // 폴더는 결과에 포함하지 않음
    } else if options.search_scope == "2" && !metadata.is_dir() {
        return Ok(()); // 파일은 결과에 포함하지 않음
    }

    // 파일명 또는 폴더명 검색
    let mut is_match = file_name.contains(keyword);

    // 파일 내용 검색
    if metadata.is_file() && options.custom_file_cont_use && is_text_file(path) {
        let content = async_fs::read_to_string(&path).await.unwrap_or_else(|_| String::new());
        if content.contains(keyword) {
            is_match = true;
        }
    }

    // 매칭된 경우 전송
    if is_match {
        let file_item = FileItem {
            file_name: file_name.to_string(),
            file_path: path.to_string_lossy().to_string(),
        };

        let tx_lock = tx.lock().await;
        tx_lock.send(file_item).await.unwrap();
        println!("File or directory matched");
    }

    Ok(())
}


async fn search_with_regex(
    path: &Path,
    keyword: &str,
    options: &SearchOptions,
    metadata: &fs::Metadata,
    tx: &Arc<Mutex<Sender<FileItem>>>,
) -> Result<(), String> {
    println!("Performing regex search on: {:?}", path);
    Ok(())
}

async fn search_with_fuzzy(
    path: &Path,
    keyword: &str,
    options: &SearchOptions,
    metadata: &fs::Metadata,
    tx: &Arc<Mutex<Sender<FileItem>>>,
) -> Result<(), String> {
    println!("Performing fuzzy search on: {:?}", path);
    Ok(())
}

async fn search_with_index(
    path: &Path,
    keyword: &str,
    options: &SearchOptions,
    metadata: &fs::Metadata,
    tx: &Arc<Mutex<Sender<FileItem>>>,
) -> Result<(), String> {
    println!("Performing index-based search on: {:?}", path);
    Ok(())
}



