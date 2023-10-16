use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Page {
    total_pages: i32,
    expenses: Vec<Expense>,
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: i32,
    label: String,
}

#[derive(Serialize, Deserialize)]
pub struct Expense {
    id: i32,
    value: f64,
    name: String,
    description: Option<String>,
    link: Option<String>,
    date_created: Option<String>,
    categories: Vec<Category>,
    recur_type: Option<String>,
    recur_end: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ExpenseCategory {
    expense_id: i32,
    category_id: i32,
}

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
        -- todo remove the drop statements once ready
        DROP TABLE IF EXISTS expense_category;
        DROP TABLE IF EXISTS expense;
        DROP TABLE IF EXISTS category;

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

pub fn read_tables(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT name FROM sqlite_schema WHERE type='table';")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let item: String = row.get(0)?;
        items.push(item);
    }
    Ok(items)
}

pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        INSERT INTO category (label)
        VALUES (:label)
        ON CONFLICT (label) IGNORE -- safe to ignore already exists
        ",
        named_params! {
            ":label": label.to_lowercase().trim(),
        },
    )?;
    Ok(())
}

pub fn query_page(
    db: &Connection,
    page_size: i32,
    current_page: i32,
) -> Result<Page, rusqlite::Error> {
    let count: i32 = db.query_row("SELECT COUNT(*) FROM expense", [], |row| row.get(0))?;
    let total_pages = if count == 0 {
        1
    } else {
        (count as f64 / page_size as f64).ceil() as i32
    };

    let expenses: Vec<Expense> = Vec::new();

    let mut statement = db.prepare(
        "SELECT * FROM expense LIMIT :page_size OFFSET :offset",
    )?;
    let expenses_iter = statement.query_map(
        named_params! {
            ":page_size": page_size,
            ":offset": (current_page - 1) * page_size,
        }, |row| {
         Ok(Expense {
             id: row.get(0)?,
             value: row.get(1)?,
             name: row.get(2)?,
             description: row.get(3)?,
             link: row.get(4)?,
             date_created: row.get(5)?,
             categories: Vec::new(),
             recur_type: row.get(6)?,
             recur_end: row.get(7)?,
         })
     }
     )?;


    Ok(Page {
        total_pages,
        expenses: expenses_iter.collect::<Result<Vec<Expense>, rusqlite::Error>>()?,
    })
}

pub fn insert_expense(
    db: &Connection,
    value: f64,
    name: &str,
    description: Option<&str>,
    link: Option<&str>,
    date_created: Option<&str>,
    recur_type: Option<&str>,
    recur_end: Option<&str>,
    expense_categories: Vec<i32>,
) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        INSERT INTO expense (
            value,
            name,
            description,
            link,
            date_created,
            recur_type,
            recur_end
        )
        VALUES (
            :value,
            :name,
            :description,
            :link,
            :date_created,
            :recur_type,
            :recur_end
        )
        ",
        named_params! {
            ":value": value,
            ":name": name,
            ":description": description,
            ":link": link,
            ":date_created": date_created,
            ":recur_type": recur_type,
            ":recur_end": recur_end,
        },
    )?;
    let expense_id = db.last_insert_rowid();
    for category in expense_categories {
        db.execute(
            "INSERT INTO expense_category (expense_id, category_id) VALUES (:expense_id, :category_id)",
            named_params! {
                ":expense_id": expense_id,
                ":category_id": category,
            }
        )?;
    }
    Ok(())
}
