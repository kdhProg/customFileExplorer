use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use std::path::Path;

const JSON_PATH: &str = "../backend_properties/file_category/file_category.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    num: u32,
    name: String,
    description: String,
    color: String,
    list: Vec<String>,
}

#[tauri::command]
pub fn get_categories() -> Vec<Category> {
    let data = fs::read_to_string(JSON_PATH).unwrap_or("[]".to_string());
    let mut categories: Vec<Category> = serde_json::from_str(&data).unwrap_or(vec![]);

    for category in &mut categories {
        category.list.retain(|path| dir_exists(path));
    }

    let _ = save_categories(&categories);
    categories
}

fn dir_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn save_categories(categories: &Vec<Category>) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(categories)?;
    if Path::new(JSON_PATH).exists() {
        let mut file = OpenOptions::new().write(true).truncate(true).open(JSON_PATH)?;
        file.write_all(data.as_bytes())?;
    }
    Ok(())
}

#[tauri::command]
pub fn create_category(name: String, description: String, color: String) -> Vec<Category> {
    let mut categories = get_categories();

    let new_num = categories.iter().map(|c| c.num).max().unwrap_or(0) + 1;

    let new_category = Category {
        num: new_num,
        name,
        description,
        color,
        list: vec![],
    };

    categories.push(new_category);

    let _ = save_categories(&categories);

    categories
}


#[tauri::command]
pub fn delete_category(num: u32) -> Vec<Category> {
    let mut categories = get_categories();
    categories.retain(|c| c.num != num);
    let _ = save_categories(&categories);
    categories
}

#[tauri::command]
pub fn add_to_category(num: u32, path: String) -> Vec<Category> {
    let mut categories = get_categories();
    if let Some(category) = categories.iter_mut().find(|c| c.num == num) {
        if !category.list.contains(&path) {
            category.list.push(path);
        }
    }
    let _ = save_categories(&categories);
    categories
}

#[tauri::command]
pub fn remove_from_category(num: u32, path: String) -> Vec<Category> {
    let mut categories = get_categories();
    if let Some(category) = categories.iter_mut().find(|c| c.num == num) {
        category.list.retain(|p| p != &path);
    }
    let _ = save_categories(&categories);
    categories
}




