use rusqlite::Connection;
use std::fs;
use tauri::AppHandle;

pub fn init_db(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    println!("App data dir: {:?}", app_dir);
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("application.db");
    let mut db = Connection::open(sqlite_path)?;
    init_tables(&mut db)?;
    Ok(db)
}

pub fn init_tables(db: &mut Connection) -> Result<(), rusqlite::Error> {
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS category(
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          label TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS expense(
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          value REAL NOT NULL, -- monetary value, no currency based on users locale
          name TEXT NOT NULL, -- mandatory name of expense
          description TEXT, -- optional text
          link TEXT, -- optional link
          date_created TEXT, -- start of recur if type is non null
          recur_type TEXT, -- governed by business logic
          recur_end TEXT -- should be ignored after this date, if undefined the recur is still ongoing
        );

        CREATE TABLE IF NOT EXISTS expense_category(
          expense_id INTEGER NOT NULL,
          category_id INTEGER NOT NULL,
          PRIMARY KEY (expense_id, category_id),
          FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE CASCADE,
          FOREIGN KEY (expense_id) REFERENCES expense(id) ON DELETE CASCADE
        );
        "
    )?;
    Ok(())
}
