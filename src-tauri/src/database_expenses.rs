use rusqlite::{named_params, Connection};

use crate::dto::{Expense, Page};

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

    let mut expense_category_labels = crate::database_shared::query_expense_category_rows(db)?;

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
            categories
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

pub fn delete_expense(db: &Connection, id: i32) -> Result<(), rusqlite::Error> {
    db.execute(
        "DELETE FROM expense WHERE id = :id",
        named_params! {
            ":id": id,
        },
    )?;

    Ok(())
}
