use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FormatTypes {
    Exponential,
    Decimal,
    Percentage,
    Time,
    Currency,
    Date
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectArea {
    pub id: i64,  // Changed from String to i64 to match int8 in database
    pub created_at: Option<String>,  // Added to match database schema
    pub title: String,
    pub category: String,
    pub desc: Option<String>,
    pub order: Option<i32>,
    pub format: Option<FormatTypes>,  // Changed to use FormatTypes enum
}



impl ProjectArea {
    pub fn to_dto(&self) -> ProjectAreaDto {
        ProjectAreaDto {
            title: self.title.clone(),
            category: self.category.clone(),
            desc: self.desc.clone(),
            order: self.order,
            format: self.format.clone(),
        }
    }
    pub fn to_format(&self) -> String {
        if let Some(format) = &self.format {
            match format {
                FormatTypes::Exponential => {
                    let exponent = self.title.parse::<i32>().unwrap_or(0);
                    if exponent == 0 {
                        format!("<var>1</var>&nbsp;m")
                    } else {
                        format!("<var>10<sup>{}</sup></var>&nbsp;m", exponent)
                    }
                },
                FormatTypes::Decimal => format!("<span>{}</span>", self.title),
                FormatTypes::Percentage => format!("<span>{}</span>", self.title),
                FormatTypes::Time => format!("<span>{}</span>", self.title),
                FormatTypes::Currency => format!("<span>{}</span>", self.title),
                FormatTypes::Date => format!("<span>{}</span>", self.title),
            }
        } else {
            format!("<span>{}</span>", self.title)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectAreaDto {
    pub title: String,
    pub desc: Option<String>,
    pub category: String,
    pub order: Option<i32>,
    pub format: Option<FormatTypes>,
}

impl ProjectAreaDto {
    pub fn from_category(category: String) -> Self {
        ProjectAreaDto {
            title: String::new(),
            desc: None,
            category,
            order: None,
            format: None,
        }
    }
}

