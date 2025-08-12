#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub struct ProjectDatabase {
    projects: Vec<Project>,
}

impl ProjectDatabase {
    pub fn new() -> Self {
        ProjectDatabase {
            projects: vec![
        Project {
            id: 1,
            title: "Aircraft engine blades".to_string(),
            description: "Precision inspection of aircraft engine blades".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(500000),
            status: ProjectStatus::Completed,
            date_started: Some("2023-03-15".to_string()),
            date_ended: Some("2024-08-22".to_string()),
            precision: Some(-4),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("SR Technics".to_string()),
            industry: Industry::Aerospace,
            maturity: Maturity::Product,
        },
        Project {
            id: 2,
            title: "Gas turbine".to_string(),
            description: "Gas turbine inspection and monitoring system".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::SpatialAwareness],
            budget: Some(750000),
            status: ProjectStatus::InProgress,
            date_started: Some("2023-01-10".to_string()),
            date_ended: None,
            precision: Some(-3),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses, Tool::Hololens],
            client: Some("ANSALDO".to_string()),
            industry: Industry::Energy,
            maturity: Maturity::Poc,
        },
        Project {
            id: 3,
            title: "Wind turbine blades".to_string(),
            description: "Wind turbine blade inspection capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(320000),
            status: ProjectStatus::Completed,
            date_started: Some("2022-11-05".to_string()),
            date_ended: Some("2024-02-28".to_string()),
            precision: Some(-2),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("nordex".to_string()),
            industry: Industry::Energy,
            maturity: Maturity::Product,
        },
        Project {
            id: 4,
            title: "Assembly line QC".to_string(),
            description: "Assembly line quality control system".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection],
            budget: Some(180000),
            status: ProjectStatus::OnHold,
            date_started: Some("2023-07-20".to_string()),
            date_ended: None,
            precision: Some(-3),
            tools: vec![Tool::Mobile, Tool::DslrCamera],
            client: Some("manufacturing corp".to_string()),
            industry: Industry::Manufacturing,
            maturity: Maturity::Research,
        },
        Project {
            id: 5,
            title: "Solar panel defects".to_string(),
            description: "Solar panel defect detection system".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(220000),
            status: ProjectStatus::Completed,
            date_started: Some("2023-05-12".to_string()),
            date_ended: Some("2024-01-18".to_string()),
            precision: Some(-2),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("solar tech".to_string()),
            industry: Industry::Energy,
            maturity: Maturity::Product,
        },
        Project {
            id: 6,
            title: "Robotic arms visual guidance".to_string(),
            description: "Visual guidance system for robotic arms".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::SpatialAwareness, Technology::AugmentedReality],
            budget: Some(290000),
            status: ProjectStatus::InProgress,
            date_started: Some("2024-02-01".to_string()),
            date_ended: None,
            precision: Some(-3),
            tools: vec![Tool::Mobile, Tool::Hololens],
            client: Some("robotics inc".to_string()),
            industry: Industry::Manufacturing,
            maturity: Maturity::Poc,
        },
        Project {
            id: 7,
            title: "PCB missing components".to_string(),
            description: "PCB component detection capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(85000),
            status: ProjectStatus::Completed,
            date_started: Some("2023-11-20".to_string()),
            date_ended: Some("2024-05-15".to_string()),
            precision: Some(-3),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("na".to_string()),
            industry: Industry::Electronics,
            maturity: Maturity::Research,
        },
        Project {
            id: 8,
            title: "IR thermal imaging".to_string(),
            description: "Infrared thermal imaging capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(135000),
            status: ProjectStatus::InProgress,
            date_started: Some("2024-01-08".to_string()),
            date_ended: None,
            precision: Some(-2),
            tools: vec![Tool::DslrCamera],
            client: Some("nordex".to_string()),
            industry: Industry::Energy,
            maturity: Maturity::Research,
        },
        Project {
            id: 9,
            title: "Telescope antennas".to_string(),
            description: "Telescope antenna capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AugmentedReality, Technology::SpatialAwareness],
            budget: Some(320000),
            status: ProjectStatus::Completed,
            date_started: Some("2022-08-15".to_string()),
            date_ended: Some("2024-03-20".to_string()),
            precision: Some(-1),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("telescope".to_string()),
            industry: Industry::Aerospace,
            maturity: Maturity::Product,
        },
        Project {
            id: 10,
            title: "Construction site tracking".to_string(),
            description: "Construction site tracking capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::SpatialAwareness],
            budget: Some(200000),
            status: ProjectStatus::Planning,
            date_started: Some("2024-05-25".to_string()),
            date_ended: None,
            precision: Some(0),
            tools: vec![Tool::DslrCamera],
            client: Some("marketing".to_string()),
            industry: Industry::Construction,
            maturity: Maturity::Research,
        },
        Project {
            id: 11,
            title: "blade MRO (Nahuel)".to_string(),
            description: "Blade maintenance, repair, and overhaul capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(420000),
            status: ProjectStatus::Completed,
            date_started: Some("2023-04-10".to_string()),
            date_ended: Some("2024-06-28".to_string()),
            precision: Some(-4),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("SR Technics".to_string()),
            industry: Industry::Aerospace,
            maturity: Maturity::Poc,
        },
        Project {
            id: 12,
            title: "BSI".to_string(),
            description: "BSI capability project".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality, Technology::Llm],
            budget: Some(150000),
            status: ProjectStatus::InProgress,
            date_started: Some("2024-02-28".to_string()),
            date_ended: None,
            precision: Some(-2),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("C:\\roboscope".to_string()),
            industry: Industry::Industrial,
            maturity: Maturity::Research,
        },
        Project {
            id: 13,
            title: "Screw and bolt feature recognition".to_string(),
            description: "Screw and bolt feature recognition capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(95000),
            status: ProjectStatus::OnHold,
            date_started: Some("2023-09-12".to_string()),
            date_ended: None,
            precision: Some(-4),
            tools: vec![Tool::Mobile, Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("marketing".to_string()),
            industry: Industry::Manufacturing,
            maturity: Maturity::Research,
        },
        Project {
            id: 14,
            title: "Pipeline monitoring".to_string(),
            description: "Pipeline monitoring and inspection capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::AugmentedReality],
            budget: Some(280000),
            status: ProjectStatus::Completed,
            date_started: Some("2023-06-01".to_string()),
            date_ended: Some("2024-04-12".to_string()),
            precision: Some(-2),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("northstream".to_string()),
            industry: Industry::Energy,
            maturity: Maturity::Poc,
        },
        Project {
            id: 15,
            title: "Training material generation".to_string(),
            description: "AI-powered training material generation capability".to_string(),
            technologies: vec![Technology::Llm, Technology::AugmentedReality, Technology::ComputerVision],
            budget: Some(110000),
            status: ProjectStatus::InProgress,
            date_started: Some("2024-03-15".to_string()),
            date_ended: None,
            precision: Some(-2),
            tools: vec![Tool::Mobile, Tool::Hololens],
            client: Some("marketing".to_string()),
            industry: Industry::Training,
            maturity: Maturity::Research,
        },
        Project {
            id: 16,
            title: "Quality control automation".to_string(),
            description: "Automated quality control inspection capability".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection],
            budget: Some(165000),
            status: ProjectStatus::Completed,
            date_started: Some("2023-10-08".to_string()),
            date_ended: Some("2024-07-30".to_string()),
            precision: Some(-3),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("na".to_string()),
            industry: Industry::Manufacturing,
            maturity: Maturity::Product,
        },
        Project {
            id: 17,
            title: "Predictive maintenance AR".to_string(),
            description: "Augmented reality predictive maintenance system".to_string(),
            technologies: vec![Technology::AugmentedReality, Technology::AiDetection, Technology::SpatialAwareness],
            budget: Some(245000),
            status: ProjectStatus::Planning,
            date_started: Some("2024-08-01".to_string()),
            date_ended: None,
            precision: Some(-2),
            tools: vec![Tool::Hololens, Tool::Mobile],
            client: Some("industrial corp".to_string()),
            industry: Industry::Industrial,
            maturity: Maturity::Research,
        },
        Project {
            id: 18,
            title: "Drone inspection platform".to_string(),
            description: "Automated drone-based inspection platform".to_string(),
            technologies: vec![Technology::ComputerVision, Technology::AiDetection, Technology::SpatialAwareness],
            budget: Some(380000),
            status: ProjectStatus::InProgress,
            date_started: Some("2024-01-22".to_string()),
            date_ended: None,
            precision: Some(0),
            tools: vec![Tool::DslrCamera, Tool::HiZoomLenses],
            client: Some("aviation tech".to_string()),
            industry: Industry::Aviation,
            maturity: Maturity::Poc,
        },
            ],
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
