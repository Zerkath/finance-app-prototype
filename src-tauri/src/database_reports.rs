use crate::dto::{BasicReport, ReportType};
use rusqlite::{named_params, Connection};
use std::collections::HashMap;

pub fn get_supported_report_types() -> Result<Vec<ReportType>, ()> {
    Ok(vec![ReportType::MONTH, ReportType::YEAR])
}

pub fn get_basic_report(
    db: &Connection,
    report_type: ReportType,
    selected_date: String,
) -> Result<BasicReport, rusqlite::Error> {
    // expected date is of format YYYY-MM-DD
    let date_parts: Vec<&str> = selected_date.split("-").collect();

    let date = match (date_parts.as_slice(), report_type) {
        ([year, _, _], ReportType::YEAR) => {
            format!("{}-__-__", year)
        }
        ([year, month, _], ReportType::MONTH) => {
            format!("{}-{}-__", year, month)
        }
        _ => panic!("Invalid date format"),
    };

    let mut date_statement = db.prepare(
        "
        SELECT SUM(e.value), e.date_created FROM expense e
        WHERE e.date_created LIKE (:date)
        GROUP BY e.date_created
        ",
    )?;

    let mut date_rows = date_statement.query(named_params! {
        ":date": date,
    })?;

    let mut dates: HashMap<String, f64> = HashMap::new();

    while let Some(row) = date_rows.next()? {
        let sum: f64 = row.get(0)?;
        let date_created: String = row.get(1)?;
        dates.insert(date_created, sum);
    }

    let mut total: f64 = 0.0;
    for (_, sum) in &dates {
        total += sum;
    }

    let mut categories_statement = db.prepare(
        "
        WITH grouped_category AS (
            SELECT 
              ec.expense_id as expense_id,
              group_concat(c.id, ', ') as ids,
              group_concat(c.label, ', ') as labels
            FROM category c
            INNER JOIN expense_category ec ON ec.category_id = c.id
            GROUP BY ec.expense_id
        )

        SELECT gc.labels, SUM(e.value)
        FROM expense e
        INNER JOIN grouped_category gc ON gc.expense_id = e.id
        WHERE e.date_created LIKE (:date)
        GROUP BY e.date_created, gc.labels
        ",
    )?;

    let mut categories_rows = categories_statement.query(named_params! {
        ":date": date,
    })?;

    let mut categories: HashMap<String, f64> = HashMap::new();

    while let Some(row) = categories_rows.next()? {
        let labels: String = row.get(0)?;
        let sum: f64 = row.get(1)?;
        categories.insert(labels, sum);
    }

    let mut uncategorized = total;

    for (_, sum) in &categories {
        uncategorized -= sum;
    }

    let report = BasicReport {
        total,
        uncategorized,
        dates,
        categories,
    };

    Ok(report)
}
