// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod commands;
mod sch_adv_properties_slot;

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
    commands::open_file_with_default_program,
    commands::get_drive_info,
    commands::read_json_file,
    commands::save_util_buttons,

    sch_adv_properties_slot::save_settings,
    sch_adv_properties_slot::load_settings,
    sch_adv_properties_slot::delete_settings,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


