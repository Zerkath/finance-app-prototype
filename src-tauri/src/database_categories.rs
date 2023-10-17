use rusqlite::{named_params, Connection};

use crate::dto::Category;

pub fn upsert_category(db: &Connection, label: &str) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        INSERT INTO category (label)
        VALUES (:label)
        ON CONFLICT (label) DO NOTHING;
        ",
        named_params! {
            ":label": label.to_lowercase().trim(),
        },
    )?;
    Ok(())
}


pub fn update_category_label(db: &Connection, label: &str, id: i32) -> Result<(), rusqlite::Error> {
    db.execute("UPDATE category SET label = (:label) WHERE id = (:id)",
        named_params! {
            ":label": label.to_lowercase().trim(),
            ":id": id,
        },
    )?;
    Ok(())
}

pub fn get_categories(db: &Connection) -> Result<Vec<Category>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, label FROM category ORDER BY label ASC")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let item = Category {
            id: row.get(0)?,
            label: row.get(1)?,
        };
        items.push(item);
    }
    Ok(items)
}

