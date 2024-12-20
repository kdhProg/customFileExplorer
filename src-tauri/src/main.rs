// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::State;
// use std::sync::Arc;
// use once_cell::sync::OnceCell;

// use rayon::ThreadPoolBuilder;

mod commands;
mod search;
mod sch_adv_properties_slot;
mod front_utils;
mod custom_category;

use search::AppState;

// use search::SearchProcess;
// use std::sync::Arc;

fn main() {
 tauri::Builder::default()
   .manage(AppState::new())
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
     commands::create_new_item,
     commands::rename_file_or_directory,
     commands::get_keywords,

     sch_adv_properties_slot::save_settings,
     sch_adv_properties_slot::load_settings,
     sch_adv_properties_slot::delete_settings,

     search::search_files,
     search::cancel_search,

     front_utils::paste_files,
     front_utils::move_files_to_trash,

     custom_category::get_categories,
     custom_category::delete_category,
     custom_category::add_to_category,
     custom_category::remove_from_category,
     custom_category::create_category
   ])
   .run(tauri::generate_context!())
   .expect("error while running tauri application");
}

