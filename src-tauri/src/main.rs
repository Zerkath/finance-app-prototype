// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database_categories;
mod database_expenses;
mod database_migrate;
mod database_reports;
mod database_shared;
mod dto;
mod state;

use dto::{Page, ReportType};

use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn reset_tables(app_handle: AppHandle) -> Result<(), String> {
    app_handle
        .db(|db| database_migrate::reset_tables(db))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn upsert_category(app_handle: AppHandle, label: String) -> Result<(), String> {
    app_handle
        .db(|db| database_categories::upsert_category(db, &label))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_category_label(app_handle: AppHandle, label: String, id: i32) -> Result<(), String> {
    app_handle
        .db(|db| database_categories::update_category_label(db, &label, id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_categories(app_handle: AppHandle) -> Result<Vec<dto::Category>, String> {
    app_handle
        .db(|db| database_categories::get_categories(db))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_category(app_handle: AppHandle, id: i32) -> Result<(), String> {
    app_handle
        .db(|db| database_categories::delete_category(db, id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_expense(app_handle: AppHandle, id: i32) -> Result<(), String> {
    app_handle
        .db(|db| database_expenses::delete_expense(db, id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn insert_expense(
    app_handle: AppHandle,
    value: f64,
    name: &str,
    description: Option<&str>,
    link: Option<&str>,
    date_created: Option<&str>,
    expense_categories: Vec<i32>,
) -> Result<(), String> {
    app_handle
        .db(|db| {
            database_expenses::insert_expense(
                db,
                value,
                name,
                description,
                link,
                date_created,
                expense_categories,
            )
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn query_page(app_handle: AppHandle, page_size: i32, current_page: i32) -> Result<Page, String> {
    app_handle
        .db(|db| database_expenses::query_page(db, page_size, current_page))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_supported_report_types(app_handle: AppHandle) -> Result<Vec<ReportType>, ()> {
    app_handle.db(|_| database_reports::get_supported_report_types())
}

#[tauri::command]
fn get_basic_report(
    app_handle: AppHandle,
    report_type: ReportType,
    selected_date: String,
) -> Result<dto::BasicReport, String> {
    app_handle
        .db(|db| database_reports::get_basic_report(db, report_type, selected_date))
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            upsert_category,
            delete_category,
            delete_expense,
            insert_expense,
            update_category_label,
            get_categories,
            query_page,
            reset_tables,
            get_supported_report_types,
            get_basic_report,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_supported_api() -> Result<(), ()> {
        // this is a useless test, just verifying that test api is working
        let types = database_reports::get_supported_report_types()?;
        let expected = vec![ReportType::MONTH, ReportType::YEAR];
        if types.iter().all(|e| expected.contains(e)) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    fn verify_basic_report_initial_state() -> Result<(), String> {

        let mut db = database_migrate::init_db_in_memory().map_err(|e| e.to_string())?;

        let report = database_reports::get_basic_report(
            &mut db,
            ReportType::MONTH,
            "2020-01-01".to_string(),
        )
        .map_err(|e| e.to_string())?;

        assert_eq!(report.total, 0.0);
        assert_eq!(report.uncategorized, 0.0);
        assert_eq!(report.dates.len(), 0);
        assert_eq!(report.categories.len(), 0);

        Ok(())
    }
}
