use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectAreaLink {
    pub id: i64,  // Changed from String to i64 to match int8 in database
    pub created_at: Option<String>,  // Added to match database schema
    pub project_id: i64,
    pub area_id: i64,    
}

