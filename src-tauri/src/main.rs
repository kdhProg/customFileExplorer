// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::State;
// use std::sync::Arc;
// use once_cell::sync::OnceCell;

// use rayon::ThreadPoolBuilder;

mod commands;
mod search;
mod sch_adv_properties_slot;

use search::AppState;

use search::SearchProcess;
use std::sync::Arc;

fn main() {
 tauri::Builder::default()
   .manage(AppState::new()) // SearchProcess 관리
   .invoke_handler(tauri::generate_handler![
     commands::get_current_dir,
     commands::list_files_in_directory,
     commands::get_file_metadata,
     commands::create_new_folder,
     commands::move_to_trash,
     commands::is_directory,
     commands::open_file_with_default_program,
     commands::get_drive_info,
     commands::read_json_file,
     commands::save_util_buttons,
     commands::path_exists,

     sch_adv_properties_slot::save_settings,
     sch_adv_properties_slot::load_settings,
     sch_adv_properties_slot::delete_settings,

     search::search_files,
     search::cancel_search
   ])
   .run(tauri::generate_context!())
   .expect("error while running tauri application");
}

