// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod commands;

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    commands::get_current_dir,
    commands::list_files_in_directory,
    commands::get_file_metadata,
    commands::create_new_folder,
    commands::search_files,
    commands::move_to_trash,
    commands::is_directory,
    commands::open_file_with_default_program
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


