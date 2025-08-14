use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectArea {
    pub id: String,
    pub title: String,
    pub desc: Option<String>,
}
