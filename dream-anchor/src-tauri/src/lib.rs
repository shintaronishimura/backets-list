pub mod models;
pub mod repository;

use crate::models::BucketItem;
use crate::repository::DreamRepository;
use std::sync::Mutex;
use tauri::{Manager, State};
use uuid::Uuid;
use chrono::Utc;

pub struct AppState {
    pub repo: Mutex<DreamRepository>,
}

#[tauri::command]
async fn get_items(state: State<'_, AppState>) -> Result<Vec<BucketItem>, String> {
    let repo = state.repo.lock().unwrap();
    Ok(repo.load_items())
}

#[tauri::command]
async fn add_item(
    state: State<'_, AppState>,
    title: String,
    category: String,
    future_message: String,
) -> Result<BucketItem, String> {
    let repo = state.repo.lock().unwrap();
    let mut items = repo.load_items();

    let new_item = BucketItem {
        id: Uuid::new_v4().to_string(),
        title,
        category,
        status: "active".to_string(),
        created_at: Utc::now(),
        last_touched_at: Utc::now(),
        future_message,
        photos: Vec::new(),
    };

    items.push(new_item.clone());
    repo.save_items(&items, &format!("Add: {}", new_item.title))?;

    Ok(new_item)
}

#[tauri::command]
async fn update_item_status(
    state: State<'_, AppState>,
    id: String,
    status: String,
) -> Result<(), String> {
    let repo = state.repo.lock().unwrap();
    let mut items = repo.load_items();
    let mut title_for_msg = String::new();

    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.status = status.clone();
        item.last_touched_at = Utc::now();
        title_for_msg = item.title.clone();
    } else {
        return Err("Item not found".to_string());
    }

    repo.save_items(&items, &format!("Update Status: {} to {}", title_for_msg, status))?;
    Ok(())
}

#[tauri::command]
async fn touch_item(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let repo = state.repo.lock().unwrap();
    let mut items = repo.load_items();
    let mut title_for_msg = String::new();

    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.last_touched_at = Utc::now();
        title_for_msg = item.title.clone();
    } else {
        return Err("Item not found".to_string());
    }

    repo.save_items(&items, &format!("Touch: {}", title_for_msg))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| {
                // Fallback for development if app_data_dir fails
                PathBuf::from("./data")
            });
            let repo = DreamRepository::new(app_data_dir);
            app.manage(AppState {
                repo: Mutex::new(repo),
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_items,
            add_item,
            update_item_status,
            touch_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::path::PathBuf;
