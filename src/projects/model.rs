use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub desc: Option<String>,
    pub created_at: Option<String>,
    pub order: Option<i32>,
}

impl Project {
    pub fn to_dto(&self) -> ProjectDto {
        ProjectDto {
            title: self.title.clone(),
            desc: self.desc.clone(),
            order: self.order.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectDto {
    pub title: String,
    pub desc: Option<String>,
    pub order: Option<i32>,
}