use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectContent {
    pub id: i64,  // Changed from String to i64 to match int8 in database
    pub created_at: Option<String>,  // Added to match database schema
    pub text: Option<String>,
    pub project_id: i64,    
}

impl ProjectContent {
    pub fn into_dto(&self) -> ProjectContentDto {
        ProjectContentDto {
            text: self.text.clone(),
            project_id: self.project_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectContentDto {
    pub text: Option<String>,
    pub project_id: i64,    
}

