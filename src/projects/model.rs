use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Technology {
    #[serde(rename = "Computer vision")]
    ComputerVision,
    #[serde(rename = "AI Detection")]
    AiDetection,
    #[serde(rename = "LLM")]
    Llm,
    #[serde(rename = "Augmented reality")]
    AugmentedReality,
    #[serde(rename = "Spatial awareness")]
    SpatialAwareness,
}

impl Technology {
     pub fn iter() -> impl Iterator<Item = Technology> {
        [
            Technology::ComputerVision,
            Technology::AiDetection,
            Technology::Llm,
            Technology::AugmentedReality,
            Technology::SpatialAwareness,
        ].into_iter()
    }

    pub fn to_string(&self) -> String {
        match self {
            Technology::ComputerVision => "Computer vision".to_string(),
            Technology::AiDetection => "AI Detection".to_string(),
            Technology::Llm => "LLM".to_string(),
            Technology::AugmentedReality => "Augmented reality".to_string(),
            Technology::SpatialAwareness => "Spatial awareness".to_string(),
        }
    }
}

impl fmt::Display for Technology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Technology::ComputerVision => "Computer vision",
            Technology::AiDetection => "AI Detection",
            Technology::Llm => "LLM",
            Technology::AugmentedReality => "Augmented reality",
            Technology::SpatialAwareness => "Spatial awareness",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum Tool {
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "DSLR camera")]
    DslrCamera,
    #[serde(rename = "Hi Zoom lenses")]
    HiZoomLenses,
    #[serde(rename = "Hololens")]
    Hololens,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ProjectStatus {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "on-hold")]
    OnHold,
    #[serde(rename = "planning")]
    Planning,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Industry {
    Aerospace,
    Manufacturing,
    Energy,
    Electronics,
    Construction,
    Aviation,
    Industrial,
    Training,
    Marketing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Maturity {
    Research,
    Poc,
    Product,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BudgetRange {
    #[serde(rename = "Under 100k")]
    Under100k,    // < 100,000
    #[serde(rename = "100k-200k")]
    Range100k200k, // 100,000 - 200,000
    #[serde(rename = "200k-300k")]
    Range200k300k, // 200,000 - 300,000
    #[serde(rename = "300k-400k")]
    Range300k400k, // 300,000 - 400,000
    #[serde(rename = "400k-500k")]
    Range400k500k, // 400,000 - 500,000
    #[serde(rename = "Over 500k")]
    Over500k,     // > 500,000
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimelineRange {
    #[serde(rename = "2022")]
    Year2022,
    #[serde(rename = "2023")]
    Year2023,
    #[serde(rename = "2024")]
    Year2024,
    #[serde(rename = "Multi-year")]
    MultiYear,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrecisionRange {
    #[serde(rename = "Very High (-4)")]
    VeryHigh,    // -4
    #[serde(rename = "High (-3)")]
    High,        // -3
    #[serde(rename = "Medium (-2)")]
    Medium,      // -2
    #[serde(rename = "Low (-1)")]
    Low,         // -1
    #[serde(rename = "Standard (0)")]
    Standard,    // 0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClientType {
    #[serde(rename = "Aerospace & Aviation")]
    AerospaceAviation,  // SR Technics, telescope, aviation tech
    #[serde(rename = "Energy & Utilities")]
    EnergyUtilities,    // ANSALDO, nordex, northstream, solar tech
    #[serde(rename = "Manufacturing & Industrial")]
    ManufacturingIndustrial, // manufacturing corp, robotics inc, industrial corp
    #[serde(rename = "Technology & R&D")]
    TechnologyRnD,      // C:\roboscope
    #[serde(rename = "Marketing & Training")]
    MarketingTraining,  // marketing
    #[serde(rename = "Internal/Research")]
    InternalResearch,   // na
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub technologies: Vec<Technology>,
    pub budget: Option<u32>,
    pub status: ProjectStatus,
    pub date_started: Option<String>,
    pub date_ended: Option<String>,
    pub precision: Option<i32>,
    pub tools: Vec<Tool>,
    pub client: Option<String>,
    pub industry: Industry,
    pub maturity: Maturity,
}

// Implementation of iter() methods for all enums


impl Tool {
    pub fn iter() -> impl Iterator<Item = Tool> {
        [
            Tool::Mobile,
            Tool::DslrCamera,
            Tool::HiZoomLenses,
            Tool::Hololens,
        ].into_iter()
    }
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tool::Mobile => "mobile",
            Tool::DslrCamera => "DSLR camera",
            Tool::HiZoomLenses => "Hi Zoom lenses",
            Tool::Hololens => "Hololens",
        })
    }
}

impl ProjectStatus {
    pub fn iter() -> impl Iterator<Item = ProjectStatus> {
        [
            ProjectStatus::InProgress,
            ProjectStatus::Completed,
            ProjectStatus::OnHold,
            ProjectStatus::Planning,
        ].into_iter()
    }
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ProjectStatus::InProgress => "in-progress",
            ProjectStatus::Completed => "completed",
            ProjectStatus::OnHold => "on-hold",
            ProjectStatus::Planning => "planning",
        })
    }
}

impl Industry {
    pub fn iter() -> impl Iterator<Item = Industry> {
        [
            Industry::Aerospace,
            Industry::Manufacturing,
            Industry::Energy,
            Industry::Electronics,
            Industry::Construction,
            Industry::Aviation,
            Industry::Industrial,
            Industry::Training,
            Industry::Marketing,
        ].into_iter()
    }
}

impl fmt::Display for Industry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Industry::Aerospace => "Aerospace",
            Industry::Manufacturing => "Manufacturing",
            Industry::Energy => "Energy",
            Industry::Electronics => "Electronics",
            Industry::Construction => "Construction",
            Industry::Aviation => "Aviation",
            Industry::Industrial => "Industrial",
            Industry::Training => "Training",
            Industry::Marketing => "Marketing",
        })
    }
}

impl Maturity {
    pub fn iter() -> impl Iterator<Item = Maturity> {
        [
            Maturity::Research,
            Maturity::Poc,
            Maturity::Product,
        ].into_iter()
    }
}

impl fmt::Display for Maturity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Maturity::Research => "Research",
            Maturity::Poc => "PoC",
            Maturity::Product => "Product",
        })
    }
}

impl BudgetRange {
    pub fn iter() -> impl Iterator<Item = BudgetRange> {
        [
            BudgetRange::Under100k,
            BudgetRange::Range100k200k,
            BudgetRange::Range200k300k,
            BudgetRange::Range300k400k,
            BudgetRange::Range400k500k,
            BudgetRange::Over500k,
        ].into_iter()
    }

    pub fn from_budget(budget: Option<u32>) -> Option<BudgetRange> {
        match budget {
            Some(b) if b < 100_000 => Some(BudgetRange::Under100k),
            Some(b) if b < 200_000 => Some(BudgetRange::Range100k200k),
            Some(b) if b < 300_000 => Some(BudgetRange::Range200k300k),
            Some(b) if b < 400_000 => Some(BudgetRange::Range300k400k),
            Some(b) if b < 500_000 => Some(BudgetRange::Range400k500k),
            Some(_) => Some(BudgetRange::Over500k),
            None => None,
        }
    }
}

impl fmt::Display for BudgetRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            BudgetRange::Under100k => "Under 100k",
            BudgetRange::Range100k200k => "100k-200k",
            BudgetRange::Range200k300k => "200k-300k",
            BudgetRange::Range300k400k => "300k-400k",
            BudgetRange::Range400k500k => "400k-500k",
            BudgetRange::Over500k => "Over 500k",
        })
    }
}

impl TimelineRange {
    pub fn iter() -> impl Iterator<Item = TimelineRange> {
        [
            TimelineRange::Year2022,
            TimelineRange::Year2023,
            TimelineRange::Year2024,
            TimelineRange::MultiYear,
        ].into_iter()
    }
}

impl fmt::Display for TimelineRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TimelineRange::Year2022 => "2022",
            TimelineRange::Year2023 => "2023",
            TimelineRange::Year2024 => "2024",
            TimelineRange::MultiYear => "Multi-year",
        })
    }
}

impl PrecisionRange {
    pub fn iter() -> impl Iterator<Item = PrecisionRange> {
        [
            PrecisionRange::VeryHigh,
            PrecisionRange::High,
            PrecisionRange::Medium,
            PrecisionRange::Low,
            PrecisionRange::Standard,
        ].into_iter()
    }

    pub fn from_precision(precision: Option<i32>) -> Option<PrecisionRange> {
        match precision {
            Some(-4) => Some(PrecisionRange::VeryHigh),
            Some(-3) => Some(PrecisionRange::High),
            Some(-2) => Some(PrecisionRange::Medium),
            Some(-1) => Some(PrecisionRange::Low),
            Some(0) => Some(PrecisionRange::Standard),
            _ => None,
        }
    }
}

impl fmt::Display for PrecisionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PrecisionRange::VeryHigh => "Very High",
            PrecisionRange::High => "High",
            PrecisionRange::Medium => "Medium",
            PrecisionRange::Low => "Low",
            PrecisionRange::Standard => "Standard",
        })
    }
}

impl ClientType {
    pub fn iter() -> impl Iterator<Item = ClientType> {
        [
            ClientType::AerospaceAviation,
            ClientType::EnergyUtilities,
            ClientType::ManufacturingIndustrial,
            ClientType::TechnologyRnD,
            ClientType::MarketingTraining,
            ClientType::InternalResearch,
        ].into_iter()
    }

    pub fn from_client(client: Option<&String>) -> Option<ClientType> {
        match client {
            Some(c) => {
                let client_lower = c.to_lowercase();
                if client_lower.contains("sr technics") || client_lower.contains("telescope") || client_lower.contains("aviation") {
                    Some(ClientType::AerospaceAviation)
                } else if client_lower.contains("ansaldo") || client_lower.contains("nordex") || client_lower.contains("northstream") || client_lower.contains("solar") {
                    Some(ClientType::EnergyUtilities)
                } else if client_lower.contains("manufacturing") || client_lower.contains("robotics") || client_lower.contains("industrial") {
                    Some(ClientType::ManufacturingIndustrial)
                } else if client_lower.contains("roboscope") {
                    Some(ClientType::TechnologyRnD)
                } else if client_lower.contains("marketing") {
                    Some(ClientType::MarketingTraining)
                } else if client_lower.contains("na") {
                    Some(ClientType::InternalResearch)
                } else {
                    Some(ClientType::InternalResearch) // default for unknown clients
                }
            }
            None => None,
        }
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ClientType::AerospaceAviation => "Aerospace & Aviation",
            ClientType::EnergyUtilities => "Energy & Utilities",
            ClientType::ManufacturingIndustrial => "Manufacturing",
            ClientType::TechnologyRnD => "Technology R&D",
            ClientType::MarketingTraining => "Marketing & Training",
            ClientType::InternalResearch => "Internal Research",
        })
    }
}

pub struct ProjectDatabase {
    projects: Vec<Project>,
}

impl ProjectDatabase {
    pub fn new() -> Self {
        // Import the mock data from the separate module
        use super::posts_mock_data::create_mock_projects;
        ProjectDatabase {
            projects: create_mock_projects(),
        }
    }

    pub fn get_all_projects(&self) -> &Vec<Project> {
        &self.projects
    }

    pub fn get_project_by_id(&self, id: u32) -> Option<&Project> {
        self.projects.iter().find(|project| project.id == id)
    }

    pub fn get_projects_by_status(&self, status: ProjectStatus) -> Vec<&Project> {
        self.projects.iter().filter(|project| project.status == status).collect()
    }

    pub fn get_projects_by_technology(&self, technology: Technology) -> Vec<&Project> {
        self.projects.iter().filter(|project| project.technologies.contains(&technology)).collect()
    }

    pub fn get_projects_by_client(&self, client: &str) -> Vec<&Project> {
        self.projects.iter().filter(|project| {
            project.client.as_ref().map_or(false, |c| c.contains(client))
        }).collect()
    }

    pub fn get_projects_by_industry(&self, industry: Industry) -> Vec<&Project> {
        self.projects.iter().filter(|project| project.industry == industry).collect()
    }

    pub fn get_projects_by_maturity(&self, maturity: Maturity) -> Vec<&Project> {
        self.projects.iter().filter(|project| project.maturity == maturity).collect()
    }

    pub fn get_projects_by_precision_range(&self, min_precision: i32, max_precision: i32) -> Vec<&Project> {
        self.projects.iter().filter(|project| {
            project.precision.map_or(false, |p| p >= min_precision && p <= max_precision)
        }).collect()
    }

    pub fn get_projects_by_budget_range(&self, min_budget: u32, max_budget: u32) -> Vec<&Project> {
        self.projects.iter().filter(|project| {
            project.budget.map_or(false, |b| b >= min_budget && b <= max_budget)
        }).collect()
    }

    pub fn get_active_projects(&self) -> Vec<&Project> {
        self.projects.iter().filter(|project| {
            project.status == ProjectStatus::InProgress || project.status == ProjectStatus::Planning
        }).collect()
    }

    pub fn get_completed_projects(&self) -> Vec<&Project> {
        self.projects.iter().filter(|project| project.status == ProjectStatus::Completed).collect()
    }

    pub fn get_projects_statistics(&self) -> HashMap<ProjectStatus, usize> {
        let mut stats = HashMap::new();
        for project in &self.projects {
            *stats.entry(project.status.clone()).or_insert(0) += 1;
        }
        stats
    }
}

pub fn create_database() -> ProjectDatabase {
    ProjectDatabase::new()
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectComparisonArea {
    Technology,
    Precision,
    Tools,
    Client,
    Industry,
    Maturity,
    Budget,
    Timeline,
}

impl ProjectComparisonArea {
    pub fn to_string(&self) -> String {
        match self {
            ProjectComparisonArea::Technology => "Technology".to_string(),
            ProjectComparisonArea::Budget => "Budget".to_string(),
            ProjectComparisonArea::Timeline => "Timeline".to_string(),
            ProjectComparisonArea::Precision => "Precision".to_string(),
            ProjectComparisonArea::Tools => "Tools".to_string(),
            ProjectComparisonArea::Client => "Client".to_string(),
            ProjectComparisonArea::Industry => "Industry".to_string(),
            ProjectComparisonArea::Maturity => "Maturity".to_string(),
        }
    }
    pub fn iter() -> impl Iterator<Item = ProjectComparisonArea> {
        [
            ProjectComparisonArea::Technology,
            ProjectComparisonArea::Precision,
            ProjectComparisonArea::Tools,
            ProjectComparisonArea::Client,
            ProjectComparisonArea::Industry,
            ProjectComparisonArea::Maturity,
            ProjectComparisonArea::Budget,
            ProjectComparisonArea::Timeline,
        ].into_iter()
    }
   
    pub fn get_items(&self) -> Vec<Box<dyn std::fmt::Display>> {
        match self {
            ProjectComparisonArea::Technology => Technology::iter().map(|t| Box::new(t) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Budget => BudgetRange::iter().map(|b| Box::new(b) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Timeline => TimelineRange::iter().map(|t| Box::new(t) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Precision => PrecisionRange::iter().map(|p| Box::new(p) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Tools => Tool::iter().map(|t| Box::new(t) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Client => ClientType::iter().map(|c| Box::new(c) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Industry => Industry::iter().map(|i| Box::new(i) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Maturity => Maturity::iter().map(|m| Box::new(m) as Box<dyn std::fmt::Display>).collect(),
        }
    }

    pub fn get_project_area_items(&self, project: &Project) -> Vec<Box<dyn std::fmt::Display>> {
        match self {
            ProjectComparisonArea::Technology => project.technologies.iter().map(|t| Box::new(t.clone()) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Budget => {
                if let Some(budget_range) = BudgetRange::from_budget(project.budget) {
                    vec![Box::new(budget_range) as Box<dyn std::fmt::Display>]
                } else {
                    vec![]
                }
            },
            ProjectComparisonArea::Timeline => {
                // This would need date parsing logic to determine timeline range
                vec![]
            },
            ProjectComparisonArea::Precision => {
                if let Some(precision_range) = PrecisionRange::from_precision(project.precision) {
                    vec![Box::new(precision_range) as Box<dyn std::fmt::Display>]
                } else {
                    vec![]
                }
            },
            ProjectComparisonArea::Tools => project.tools.iter().map(|t| Box::new(t.clone()) as Box<dyn std::fmt::Display>).collect(),
            ProjectComparisonArea::Client => {
                if let Some(client_type) = ClientType::from_client(project.client.as_ref()) {
                    vec![Box::new(client_type) as Box<dyn std::fmt::Display>]
                } else {
                    vec![]
                }
            },
            ProjectComparisonArea::Industry => vec![Box::new(project.industry.clone()) as Box<dyn std::fmt::Display>],
            ProjectComparisonArea::Maturity => vec![Box::new(project.maturity.clone()) as Box<dyn std::fmt::Display>],
        }
    }

}
