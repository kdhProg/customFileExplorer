

use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;
use std::sync::{Arc};
use tokio::sync::{Mutex, mpsc::{self, Sender}};
use serde::Deserialize;
use std::future::Future;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, TimeZone};
use mime_guess::from_path;
use serde::{Serialize};
use tauri::State;
use tauri::Window;
use num_cpus;
use std::collections::HashSet;
use std::collections::HashMap;
use std::process::Command;
use std::env;
use windows::Win32::UI::Shell::IsUserAnAdmin;
use std::thread::ThreadId;
use regex::Regex;
use strsim::damerau_levenshtein;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;
use chrono::Local;



const CACHE_FILE_PATH: &str = "../backend_properties/cache/search_cache.json";
const CACHE_SIZE_LIMIT: usize = 50;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry {
    pub name: String,
    pub result: Vec<String>,
    pub hit: u32,
    pub search_options: SearchOptions,
}

// 1. read cache & convert into JSON
pub fn read_cache() -> Vec<CacheEntry> {
    let mut file = File::open(CACHE_FILE_PATH).unwrap_or_else(|_| File::create(CACHE_FILE_PATH).unwrap());
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    if data.is_empty() {
        return vec![];
    }
    let cache: Vec<CacheEntry> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
    cache
}

// 2. Check Cache Entries --> check search Options & Search keywords
pub fn find_in_cache(keyword: &str, current_options: &SearchOptions) -> Option<Vec<String>> {
    let cache = read_cache();
    
    let entry_opt = cache.iter().find(|entry| {
        entry.name == keyword && entry.search_options == *current_options
    });

    if let Some(entry) = entry_opt {
        return Some(entry.result.clone());
    }

    None
}




// 3. write to cache
pub fn write_cache(cache: &Vec<CacheEntry>) {
    let mut file = File::create(CACHE_FILE_PATH).unwrap();
    let data = serde_json::to_string_pretty(&cache).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

// 4. update cache
pub fn update_cache(keyword: &str, new_results: Vec<String>, current_options: &SearchOptions) {
    let mut cache = read_cache();
    
    let mut found = false;
    
    // check duplicate
    for entry in cache.iter_mut() {
        if entry.name == keyword && entry.search_options == *current_options {
            entry.hit += 1;
            entry.result = new_results.clone();
            found = true;
            break;
        }
    }

    // add new entries if not duplicated
    if !found {
        cache.push(CacheEntry {
            name: keyword.to_string(),
            result: new_results,
            hit: 1,
            search_options: current_options.clone(),
        });
    }

    // LRU
    if cache.len() > CACHE_SIZE_LIMIT {
        if let Some(min_hit_entry) = cache.iter().min_by_key(|entry| entry.hit) {
            let index = cache.iter().position(|entry| entry.name == min_hit_entry.name).unwrap();
            cache.remove(index);
        }
    }

    write_cache(&cache);
}


#[derive(Serialize)]
struct SearchLog {
    keyword: String,
    options: SearchOptions,
    directory: String,
    start_time: String,
    end_time: String,
    duration: f64,
    results: Vec<String>,
    results_count: usize,
}

fn save_log(log: &SearchLog) -> Result<(), String> {
    // Log file's name - with current timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("../logs/{}_log.json", timestamp);

    let file_path = PathBuf::from(&filename);
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    let data = serde_json::to_string_pretty(log).map_err(|e| e.to_string())?;
    file.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    
    println!("Search log saved at: {}", filename);
    Ok(())
}

fn create_search_log(
    keyword: String,
    options: SearchOptions,
    directory: String,
    start_time: Instant,
    end_time: Instant,
    results: Vec<String>,
) -> SearchLog {
    SearchLog {
        keyword,
        options,
        directory,
        start_time: format!("{}", start_time.elapsed().as_secs_f64()),
        end_time: format!("{}", end_time.elapsed().as_secs_f64()),
        duration: end_time.duration_since(start_time).as_secs_f64(),
        results_count: results.len(),
        results,
    }
}




// use by fuzzy-matching
#[derive(Deserialize, Debug)]
struct AlgorithmConfig {
    name: String,
    threshold: f64,
}

// use by fuzzy-matching
fn read_threshold_from_json(algorithm_name: &str) -> Result<f64, String> {
    let data = fs::read_to_string("../backend_properties/search_properties/fuzzy_properties.json")
        .map_err(|e| format!("Failed to read file: {}", e))?;
    println!("File read successfully: {:?}", data);

    let configs: Vec<AlgorithmConfig> = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    for config in configs {
        println!("json config name: {:?}", config.name);
        if config.name == algorithm_name {
            return Ok(config.threshold);
        }
    }

    Err(format!("Algorithm not found: {}", algorithm_name))
}



#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct FileItem {
    pub file_name: String,
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Clone,PartialEq,Debug)]
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

    #[serde(rename = "customLogUse")]
    custom_log_use: bool,
}

#[derive(Clone)]
pub struct SearchProcess {
    is_cancelled: Arc<Mutex<bool>>,
    is_completed: Arc<Mutex<bool>>,
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
        let is_cancelled = *self.is_cancelled.lock().await;
        SearchProcessInfo {
            id: self.id.clone(),
            is_cancelled,
        }
    }
}



#[derive(Serialize,Debug,Clone)]
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
    search_processes: Mutex<HashMap<String, Arc<SearchProcess>>>,
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
    state: State<'_, AppState>,
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

// for cache check if cache value is located at or under input directory
fn is_path_in_directory(file_path: &Path, dir_path: &Path) -> bool {
    let canonical_file_path = file_path.canonicalize().unwrap_or_else(|_| file_path.to_path_buf());
    let canonical_dir_path = dir_path.canonicalize().unwrap_or_else(|_| dir_path.to_path_buf());

    println!("Comparing file path: {:?}", canonical_file_path);
    println!("With directory path: {:?}", canonical_dir_path);

    if canonical_file_path.starts_with(&canonical_dir_path) {
        println!("File is within directory.");
        true
    } else {
        println!("File is NOT within directory.");
        false
    }
}




#[tauri::command]
pub async fn search_files<'a>(
    window: Window,
    keyword: String,
    directory: String,
    options: SearchOptions,
    state: State<'_, AppState>,
) -> Result<SearchProcessInfo, String> {

    let directory_clone = directory.clone(); // It will be used at making Log
    let dir_path = PathBuf::from(directory);
    
    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {:?}", dir_path));
    }

    let start_time = Instant::now();


    let cached_paths: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));


    if let Some(cached_results) = find_in_cache(&keyword, &options) {

        let mut cached_paths_lock = cached_paths.lock().await;

        for file_path in cached_results {
            let path = Path::new(&file_path);

            let is_in_directory = is_path_in_directory(path, &dir_path);

            if !path.exists() || !is_in_directory{
                continue;
            }

            let is_file = path.is_file();
            let is_dir = path.is_dir();

            match options.search_scope.as_str() {
                "1" if !is_file => continue,
                "2" if !is_dir => continue,
                _ => {},
            }

            cached_paths_lock.insert(file_path.clone());

            let file_item = FileItem {
                file_name: path.file_name().unwrap().to_string_lossy().to_string(),
                file_path: file_path.clone(),
            };

            window.emit("search-result", file_item).expect("Failed to emit search result from cache");
        }
    }


    if options.custom_thread_pool_use {
        rayon::ThreadPoolBuilder::new()
            .num_threads(options.thread_pool_num.parse::<usize>().unwrap_or_else(|_| num_cpus::get()))
            .build_global()
            .unwrap_or_else(|_| println!("Failed to set custom thread pool, using default."));
    }

    let process = Arc::new(SearchProcess::new());
    let process_info = process.get_info().await;
    let process_id = process_info.id.clone();
    println!("Search process created with ID: {}", process.id);

    state.add_process(process_id.clone(), Arc::clone(&process)).await;

    window.emit("process-info", process_info.clone()).expect("Failed to emit process info");
    println!("Backend process ID: {:?}", process.id);

    let result = Arc::new(Mutex::new(Vec::new()));
    let sent_files = Arc::new(Mutex::new(HashSet::new()));
    let (tx, mut rx) = mpsc::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    // For Check ThreadPool
    let thread_ids = Arc::new(Mutex::new(HashSet::<ThreadId>::new()));

    let process_clone = Arc::clone(&process);
    let result_clone = Arc::clone(&result);
    let tx_clone = Arc::clone(&tx);
    
    // For Check ThreadPool
    let thread_ids_clone = Arc::clone(&thread_ids);

    let keyword_for_spawn = keyword.clone();
    let options_for_spawn = options.clone();

    tokio::spawn(async move {
        let process_clone_for_cancel = Arc::clone(&process_clone);
        
        tokio::select! {
            _ = search_in_directory(dir_path, keyword_for_spawn.clone(), result_clone, process_clone, options_for_spawn, tx_clone, thread_ids_clone) => {
                println!("Search completed");
            }
            _ = async {
                loop {
                    if process_clone_for_cancel.is_cancelled().await {
                        println!("Search cancelled via select!");
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            } => {
                println!("Cancel loop executed!");
            }
        }
        drop(tx);
    });
    

    while let Some(file_item) = rx.recv().await {
        let mut sent_files_lock = sent_files.lock().await;
        
        if sent_files_lock.contains(&file_item.file_path) {
            continue;
        }
    
        if let Some(cached_results) = find_in_cache(&keyword, &options) {
            if cached_results.contains(&file_item.file_path) {
                println!("File already in cache, skipping frontend emit: {}", file_item.file_path);
            } else {
                if let Err(e) = window.emit("search-result", file_item.clone()) {
                    println!("Failed to emit search result: {:?}", e);
                }
            }
        }else {
            if let Err(e) = window.emit("search-result", file_item.clone()) {
                println!("Failed to emit search result: {:?}", e);
            }
        }
    
        sent_files_lock.insert(file_item.file_path.clone());
    
        let mut result_lock = result.lock().await;
        result_lock.push(file_item);
    }
    

    track_and_print_thread_ids(Arc::clone(&thread_ids)).await;

    process.mark_as_completed().await;
    
    let process_info = process.get_info().await;
    println!("Returning process info: {:?}", process_info);

    state.remove_process(&process_id).await;

    let final_results = {
        let result_lock = result.lock().await;
        result_lock.iter().map(|file_item| file_item.file_path.clone()).collect::<Vec<String>>()
    };
    update_cache(&keyword, final_results, &options);

     let elapsed_time = start_time.elapsed();

     window.emit("search-time", elapsed_time.as_secs_f64()).expect("Failed to emit search time");

    if options.custom_log_use {
        let final_results = {
            let result_lock = result.lock().await;
            result_lock.iter().map(|file_item| file_item.file_path.clone()).collect::<Vec<String>>()
        };

        let end_time = Instant::now();

        let log = create_search_log(
            keyword.clone(),
            options.clone(),
            directory_clone,
            start_time,
            end_time,
            final_results,
        );

        if let Err(e) = save_log(&log) {
            println!("Failed to save log: {}", e);
        }
    }



    Ok(process_info)
}





fn request_admin_privileges() -> bool {
    let executable_path = env::current_exe().expect("Failed to get current executable path.");

    // Windows powershell scripts
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process -FilePath \"{}\" -Verb runAs",
            executable_path.display()
        ))
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        println!("Admin privileges requested successfully");
        std::process::exit(0); // restart process with admin privilege
    } else {
        eprintln!("Failed to request admin privileges");
        return false;
    }
}

#[cfg(windows)]
fn can_perform_owner_based_search() -> bool {

    if cfg!(debug_assertions) {
        println!("Running in development mode, skipping admin check.");
        return true;
    }

    println!("Checking if user is admin");

    unsafe {
        if IsUserAnAdmin().as_bool() {
            return true;
        } else {
            println!("User is not admin, requesting admin privileges via UAC...");
            return request_admin_privileges();
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

    let path_str = path.to_string_lossy();
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("(Get-Acl \"{}\" | Select-Object -ExpandProperty Owner)", path_str))
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        let owner = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Some(actual_owner) = owner.split('\\').last() {
            if !actual_owner.is_empty() {
                println!("Owner found: {}", actual_owner);
                return Some(actual_owner.to_string());
            }
        }
    }

    println!("Failed to get file owner");
    None
}

fn truncate_to_date(date: DateTime<Utc>) -> DateTime<Utc> {
    let naive_date = date.naive_utc().date();
    Utc.from_utc_datetime(&naive_date.and_hms(0, 0, 0))
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
        if options.custom_file_size_use && metadata.is_file() {
            let file_size = metadata.len();
            if file_size > options.size_max || file_size < options.size_min {
                println!("File filtered by size: {}", file_size);
                return true;
            }
        }

        if options.custom_file_crt_date_use {
            if let Ok(created) = metadata.created() {
                let crt_date = truncate_to_date(DateTime::<Utc>::from(created));
        
                if let Ok(start_time) = parse_date_to_rfc3339(&options.crt_start) {
                    let start_date = truncate_to_date(start_time);
                    if crt_date < start_date {
                        println!("File or folder filtered by creation date: {:?}", crt_date);
                        return true;
                    }
                }
        
                if let Ok(end_time) = parse_date_to_rfc3339(&options.crt_end) {
                    let end_date = truncate_to_date(end_time);
                    if crt_date > end_date {
                        println!("File or folder filtered by end date: {:?}", crt_date);
                        return true;
                    }
                }
            }
        }

        if options.custom_file_modi_date_use {
            if let Ok(modified) = metadata.modified() {
                let modi_date = truncate_to_date(DateTime::<Utc>::from(modified));
        
                if let Ok(start_time) = parse_date_to_rfc3339(&options.modi_start) {
                    let start_date = truncate_to_date(start_time);
                    if modi_date < start_date {
                        println!("File or folder filtered by modified date: {:?}", modi_date);
                        return true;
                    }
                }
        
                if let Ok(end_time) = parse_date_to_rfc3339(&options.modi_end) {
                    let end_date = truncate_to_date(end_time);
                    if modi_date > end_date {
                        println!("File or folder filtered by end modified date: {:?}", modi_date);
                        return true;
                    }
                }
            }
        }
        

        

        if options.custom_file_owner_use {
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
                    return true;
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
                            continue;
                        }
                    }

                    match options.custom_sch_method.as_str() {
                        "1" => {
                            search_with_regex(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                        "2" => {
                            search_with_fuzzy_damerau_levenshtein(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                        "3" => {
                            search_with_fuzzy_jaccard_similarity(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                        _ => {
                            search_default(&path, &keyword, &options, &metadata, &tx).await?;
                        }
                    }

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
    let file_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or_default();

    if options.search_scope == "1" && metadata.is_dir() {
        return Ok(());
    } else if options.search_scope == "2" && !metadata.is_dir() {
        return Ok(());
    }

    let is_file_name_match = file_name.contains(keyword);

    let mut is_file_content_match = false;
    if metadata.is_file() && options.custom_file_cont_use && is_text_file(path) {
        let content = async_fs::read_to_string(&path).await.unwrap_or_else(|_| String::new());
        if content.contains(keyword) {
            is_file_content_match = true;
        }
    }

    if is_file_name_match || is_file_content_match {
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
    let file_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or_default();

    if options.search_scope == "1" && metadata.is_dir() {
        return Ok(());
    } else if options.search_scope == "2" && !metadata.is_dir() {
        return Ok(());
    }


    let regex = match Regex::new(keyword) {
        Ok(r) => r,
        Err(e) => return Err(format!("Invalid regex pattern: {}", e)),
    };

    let is_file_name_match = regex.is_match(file_name);
    let mut is_file_content_match = false;

    if metadata.is_file() && options.custom_file_cont_use && is_text_file(path) {
        let content = async_fs::read_to_string(&path).await.unwrap_or_else(|_| String::new());
        is_file_content_match = regex.is_match(&content);
    }

    if is_file_name_match || is_file_content_match {
        let file_item = FileItem {
            file_name: file_name.to_string(),
            file_path: path.to_string_lossy().to_string(),
        };

        let tx_lock = tx.lock().await;
        tx_lock.send(file_item).await.unwrap();
        println!("File or directory matched with regex");
    }

    Ok(())
}


async fn search_with_fuzzy_damerau_levenshtein(
    path: &Path,
    keyword: &str,
    options: &SearchOptions,
    metadata: &fs::Metadata,
    tx: &Arc<Mutex<Sender<FileItem>>>,
) -> Result<(), String> {
    println!("Performing fuzzy-based search on: {:?}", path);
    let threshold = read_threshold_from_json("Damerau-Levenshtein").unwrap_or(2.0);
    let file_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or_default();
    println!("Performing fuzzy-based Damerau-levenshtein threshold: {:?}", threshold);

    if options.search_scope == "1" && metadata.is_dir() {
        return Ok(());
    } else if options.search_scope == "2" && !metadata.is_dir() {
        return Ok(());
    }
    

    let distance = damerau_levenshtein(file_name, keyword);
    println!("distance: {:?}", distance);
    if (distance as f64) <= threshold {
        let file_item = FileItem {
            file_name: file_name.to_string(),
            file_path: path.to_string_lossy().to_string(),
        };

        let tx_lock = tx.lock().await;
        tx_lock.send(file_item).await.unwrap();
        println!("File or directory matched by fuzzy search");
    }
    
    Ok(())
}



fn jaccard_similarity(a: &str, b: &str) -> f64 {
    let a_grams: HashSet<_> = a.chars().collect();
    let b_grams: HashSet<_> = b.chars().collect();

    let intersection_size = a_grams.intersection(&b_grams).count();
    let union_size = a_grams.union(&b_grams).count();

    if union_size == 0 {
        return 0.0;
    }
    intersection_size as f64 / union_size as f64
}

async fn search_with_fuzzy_jaccard_similarity(
    path: &Path,
    keyword: &str,
    options: &SearchOptions,
    metadata: &fs::Metadata,
    tx: &Arc<Mutex<Sender<FileItem>>>,
) -> Result<(), String> {
    let file_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or_default();
    
    if options.search_scope == "1" && metadata.is_dir() {
        return Ok(());
    } else if options.search_scope == "2" && !metadata.is_dir() {
        return Ok(());
    }

    let jaccard_threshold = read_threshold_from_json("Jaccard-Similarity")
    .map(|threshold| threshold as f64)
    .unwrap_or(0.5);

    let similarity = jaccard_similarity(file_name, keyword);
    println!("Jaccard similarity between '{}' and '{}': {}", file_name, keyword, similarity);

    if similarity >= jaccard_threshold {
        let file_item = FileItem {
            file_name: file_name.to_string(),
            file_path: path.to_string_lossy().to_string(),
        };

        let tx_lock = tx.lock().await;
        tx_lock.send(file_item).await.unwrap();
        println!("File or directory matched by Jaccard similarity");
    }

    Ok(())
}



