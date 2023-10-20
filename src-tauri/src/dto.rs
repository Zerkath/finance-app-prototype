use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ReportType {
    MONTH,
    YEAR
}

#[derive(Serialize, Deserialize)]
pub struct BasicReport {
    pub total: f64,
    pub uncategorized: f64,
    pub dates: HashMap<String, f64>,
    pub categories: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub daily_reports: Vec<ReportByDay>,
    pub overall_report: ReportByCategory,
    pub total: f64,
}


#[derive(Serialize, Deserialize)]
pub struct ReportByDay {
    pub day: String,
    pub groups: Vec<ReportByCategory>,
}

#[derive(Serialize, Deserialize)]
pub struct ReportByCategory {
    pub categories: Vec<Category>,
    pub total: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub total_pages: i32,
    pub expenses: Vec<Expense>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub value: f64,
    pub name: String,
    pub description: Option<String>,
    pub date_created: Option<String>,
    pub categories: Vec<Category>
}

#[derive(Serialize, Deserialize)]
struct ExpenseCategory {
    expense_id: i32,
    category_id: i32,
}

