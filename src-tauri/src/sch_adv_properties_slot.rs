use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use tauri::command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchSettings {
    customThreadPoolUse: bool,
    threadPoolNum: String,
    searchScope: String,
    customFileContUse: bool,
    customPropertyUse: bool,
    customFileSizeUse: bool,
    customFileCrtDateUse: bool,
    customFileModiDateUse: bool,
    customFileOwnerUse: bool,
    customFileTypeUse:bool,
    sizeMax: u64,
    sizeMin: u64,
    crtStart: String,
    crtEnd: String,
    modiStart: String,
    modiEnd: String,
    ownerName: String,
    fileTypeList: String,
    customSymbolicChk: bool,
    customSchMethod: String,
    customLogUse: bool,
    fileMaxRawVal : u64,
    fileMinRawVal : u64,
    fileMaxUnit : String,
    fileMinUnit : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsData {
    data: Vec<SettingSlot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingSlot {
    number: u32,
    name: String,
    val: SearchSettings,
}


const SETTINGS_FILE: &str = "../src/properties/sch_obj_set.json";


pub fn read_settings_file() -> SettingsData {
    let mut file = File::open(SETTINGS_FILE).unwrap_or_else(|_| {
        
        let default_data = SettingsData {
            data: (1..=5)
                .map(|i| SettingSlot {
                    number: i,
                    name: String::new(),
                    val: SearchSettings {
                        customThreadPoolUse: false,
                        threadPoolNum: "0".to_string(),
                        searchScope: "0".to_string(),
                        customFileContUse: false,
                        customPropertyUse: false,
                            customFileSizeUse: false,
                            customFileCrtDateUse: false,
                            customFileModiDateUse: false,
                            customFileOwnerUse: false,
                            customFileTypeUse:false,
                        sizeMax: 0,
                        sizeMin: 0,
                        crtStart: "".to_string(),
                        crtEnd: "".to_string(),
                        modiStart: "".to_string(),
                        modiEnd: "".to_string(),
                        ownerName: "".to_string(),
                        fileTypeList: "".to_string(),
                        customSymbolicChk: false,
                        customSchMethod: "0".to_string(),
                        customLogUse: false,
                        fileMaxRawVal : 0,
                        fileMinRawVal : 0,
                        fileMaxUnit : "B".to_string(),
                        fileMinUnit : "B".to_string(),
                    },
                })
                .collect(),
        };
        let mut new_file = File::create(SETTINGS_FILE).unwrap();
        serde_json::to_writer_pretty(&mut new_file, &default_data).unwrap();
        new_file
    });

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    serde_json::from_str(&content).unwrap()
}


fn write_settings_file(data: &SettingsData) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(SETTINGS_FILE)
        .unwrap();

    let serialized_data = serde_json::to_string_pretty(data).unwrap();
    file.write_all(serialized_data.as_bytes()).unwrap();
}

#[command]
pub fn save_settings(slot_number: u32, name: String, settings: SearchSettings) -> String {
    let mut data = read_settings_file();

    if let Some(slot) = data.data.iter_mut().find(|s| s.number == slot_number) {
        slot.name = name;
        slot.val = settings;
    }

    write_settings_file(&data);
    "Settings saved successfully".to_string()
}

#[command]
pub fn load_settings(slot_number: u32) -> Option<SettingSlot> {
    let data = read_settings_file();
    data.data.into_iter().find(|s| s.number == slot_number)
}

#[command]
pub fn delete_settings(slot_number: u32) -> String {
    let mut data = read_settings_file();

    if let Some(slot) = data.data.iter_mut().find(|s| s.number == slot_number) {
        slot.name = String::new();
        slot.val = SearchSettings {
            customThreadPoolUse: false,
            threadPoolNum: "0".to_string(),
            searchScope: "0".to_string(),
            customFileContUse: false,
            customPropertyUse: false,
                customFileSizeUse: false,
                customFileCrtDateUse: false,
                customFileModiDateUse: false,
                customFileOwnerUse: false,
                customFileTypeUse:false,
            sizeMax: 0,
            sizeMin: 0,
            crtStart: "".to_string(),
            crtEnd: "".to_string(),
            modiStart: "".to_string(),
            modiEnd: "".to_string(),
            ownerName: "".to_string(),
            fileTypeList: "".to_string(),
            customSymbolicChk: false,
            customSchMethod: "0".to_string(),
            customLogUse: false,
            fileMaxRawVal : 0,
            fileMinRawVal : 0,
            fileMaxUnit : "B".to_string(),
            fileMinUnit : "B".to_string(),
        };
    }

    write_settings_file(&data);
    "Settings deleted successfully".to_string()
}
