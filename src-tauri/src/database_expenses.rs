use rusqlite::{named_params, Connection};
use std::collections::HashMap;

use crate::dto::{Expense, Page, Category};

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

    let mut expenses: Vec<Expense> = Vec::new();

    let mut expense_rows_statement =
        db.prepare("SELECT * FROM expense ORDER BY date_created ASC LIMIT :page_size OFFSET :offset")?;

    let mut expense_rows = expense_rows_statement.query(named_params! {
        ":page_size": page_size,
        ":offset": (current_page - 1) * page_size,
    })?;

    let mut expense_category_rows_statement = db.prepare(
        "
        SELECT ec.expense_id, ec.category_id, c.label
        FROM expense_category ec
        JOIN category c
        ON c.id = ec.category_id;
        ",
    )?;

    let mut expense_category_rows = expense_category_rows_statement.query([])?;

    let mut expense_category_labels: HashMap<i32, Vec<Category>> = HashMap::new();

    while let Some(row) = expense_category_rows.next()? {
        let expense_id: i32 = row.get(0)?;
        let category_id: i32 = row.get(1)?;
        let category_label: String = row.get(2)?;

        let category = Category {
            id: category_id,
            label: category_label,
        };

        expense_category_labels
            .entry(expense_id)
            .or_insert_with(Vec::new)
            .push(category);
    }

    while let Some(row) = expense_rows.next()? {
        let id = row.get(0)?;

        let categories = expense_category_labels
            .remove(&id)
            .unwrap_or(Vec::new());

        let expense = Expense {
            id,
            value: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            link: row.get(4)?,
            date_created: row.get(5)?,
            categories,
            recur_type: row.get(6)?,
            recur_end: row.get(7)?,
        };
        expenses.push(expense);
        
    }

    Ok(Page {
        total_pages,
        expenses,
    })
}

pub fn insert_expense(
    db: &Connection,
    value: f64,
    name: &str,
    description: Option<&str>,
    link: Option<&str>,
    date_created: Option<&str>,
    expense_categories: Vec<i32>,
) -> Result<(), rusqlite::Error> {

    db.execute(
        "
        INSERT INTO expense (
            value,
            name,
            description,
            link,
            date_created
        )
        VALUES (
            :value,
            :name,
            :description,
            :link,
            :date_created
        )
        ",
        named_params! {
            ":value": value,
            ":name": name,
            ":description": description,
            ":link": link,
            ":date_created": date_created
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

