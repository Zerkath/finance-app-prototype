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

    let mut statement = db.prepare("SELECT * FROM expense LIMIT :page_size OFFSET :offset")?;
    let mut rows = statement.query(
        named_params! {
            ":page_size": page_size,
            ":offset": (current_page - 1) * page_size,
        }
    )?;
    while let Some(row) = rows.next()? {
        let expense = Expense {
            id: row.get(0)?,
            value: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            link: row.get(4)?,
            date_created: row.get(5)?,
            categories: Vec::new(), // TODO should be filled with data from the expense_category table
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

