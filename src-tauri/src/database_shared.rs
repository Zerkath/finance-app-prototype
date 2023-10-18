use rusqlite::Connection;
use std::collections::HashMap;
use crate::dto::Category;

pub fn query_expense_category_rows(db: &Connection) -> Result<HashMap<i32, Vec<Category>>, rusqlite::Error> {

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

    Ok(expense_category_labels)
}
