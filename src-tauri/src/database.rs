use rusqlite::Connection;

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

