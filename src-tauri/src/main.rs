// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod database_migrate;
mod database_expenses;
mod database_categories;
mod dto;

use dto::Page;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

#[tauri::command]
fn query_tables(app_handle: AppHandle) -> Result<Vec<String>, String> {
    app_handle.db(|db| database::read_tables(db)).map_err(|e| e.to_string())
}

#[tauri::command]
fn upsert_category(app_handle: AppHandle, label: String) -> Result<(), String> {
    app_handle.db(|db| database_categories::upsert_category(db, &label)).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_category_label(app_handle: AppHandle, label: String, id: i32) -> Result<(), String> {
    app_handle.db(|db| database_categories::update_category_label(db, &label, id)).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_categories(app_handle: AppHandle) -> Result<Vec<dto::Category>, String> {
    app_handle.db(|db| database_categories::get_categories(db)).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_category(app_handle: AppHandle, id: i32) -> Result<(), String> {
    app_handle.db(|db| database_categories::delete_category(db, id)).map_err(|e| e.to_string())
}

#[tauri::command]
fn insert_expense(
    app_handle: AppHandle, 
    value: f64, 
    name: &str,
    description: Option<&str>,
    link: Option<&str>,
    date_created: Option<&str>,
    recur_type: Option<&str>,
    recur_end: Option<&str>,
    expense_categories: Vec<i32>,
) -> Result<(), String> {
    app_handle.db(|db| database_expenses::insert_expense(
        db,
        value,
        name,
        description,
        link,
        date_created,
        recur_type,
        recur_end,
        expense_categories,
    )).map_err(|e| e.to_string())
}

#[tauri::command]
fn query_page(
    app_handle: AppHandle,
    page_size: i32,
    current_page: i32,
) -> Result<Page, String> {
    app_handle.db(|db| database_expenses::query_page(db, page_size, current_page)).map_err(|e| e.to_string())
}


fn main() {
  tauri::Builder::default()
    .manage(AppState { db: Default::default() })
    .invoke_handler(tauri::generate_handler![
      query_tables,
      upsert_category,
      delete_category,
      insert_expense,
      update_category_label,
      get_categories,
      query_page
    ])
    .setup(|app| {
        let handle = app.handle();

        let app_state: State<AppState> = handle.state();
        let db = database_migrate::init_db(&handle).expect("Failed to initialize database");
        *app_state.db.lock().unwrap() = Some(db);
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
