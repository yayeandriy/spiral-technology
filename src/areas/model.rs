use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectArea {
    pub id: i64,  // Changed from String to i64 to match int8 in database
    pub created_at: Option<String>,  // Added to match database schema
    pub title: String,
    pub category: String,
    pub desc: Option<String>,
}



impl ProjectArea {
    pub fn to_dto(&self) -> ProjectAreaDto {
        ProjectAreaDto {
            title: self.title.clone(),
            category: self.category.clone(),
            desc: self.desc.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectAreaDto {
    pub title: String,
    pub desc: Option<String>,
    pub category: String,
}

impl ProjectAreaDto {
    pub fn from_category(category: String) -> Self {
        ProjectAreaDto {
            title: String::new(),
            desc: None,
            category,
        }
    }
}