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

    let mut statement = db.prepare(
        "
        SELECT SUM(e.value), e.date_created FROM expense e
        WHERE e.date_created LIKE (:date)
        GROUP BY e.date_created
        ",
    )?;

    let mut rows = statement.query(named_params! {
        ":date": date,
    })?;

    let mut dates: HashMap<String, f64> = HashMap::new();

    while let Some(row) = rows.next()? {
        let sum: f64 = row.get(0)?;
        let date_created: String = row.get(1)?;
        dates.insert(date_created, sum);
    }

    let mut total: f64 = 0.0;
    for (_, sum) in &dates {
        total += sum;
    }

    let report = BasicReport {
        total,
        dates,
    };

    Ok(report)
}
