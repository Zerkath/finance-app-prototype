use serde::{Deserialize, Serialize};

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
    pub link: Option<String>,
    pub date_created: Option<String>,
    pub categories: Vec<Category>,
    pub recur_type: Option<String>,
    pub recur_end: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ExpenseCategory {
    expense_id: i32,
    category_id: i32,
}

