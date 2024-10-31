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

    // 각 카테고리의 list 요소들을 검사하여 존재하지 않는 항목을 제거
    for category in &mut categories {
        category.list.retain(|path| dir_exists(path));
    }

    // 정리된 데이터를 다시 파일에 저장
    let _ = save_categories(&categories);
    categories
}

// 파일 또는 폴더가 실제로 존재하는지 확인하는 함수
fn dir_exists(path: &str) -> bool {
    Path::new(path).exists()
}

// JSON 파일에 카테고리 데이터를 저장하는 함수
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

    // 새로운 num은 현재 가장 큰 num 값 + 1
    let new_num = categories.iter().map(|c| c.num).max().unwrap_or(0) + 1;

    // 새 카테고리 객체 생성
    let new_category = Category {
        num: new_num,
        name,
        description,
        color,
        list: vec![], // 초기에는 빈 리스트로 설정
    };

    // 새 카테고리 추가
    categories.push(new_category);

    // 업데이트된 카테고리를 JSON 파일에 저장
    let _ = save_categories(&categories);

    // 변경된 카테고리 목록 반환
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




