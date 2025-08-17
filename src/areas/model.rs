use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectArea {
    pub id: i64,  // Changed from String to i64 to match int8 in database
    pub created_at: Option<String>,  // Added to match database schema
    pub title: String,
    pub category: ProjectCategoryName,
    pub desc: Option<String>,
}

pub type ProjectCategoryName = String;
