use leptos::{
    prelude::{
        provide_context, signal, use_context, Children, Effect, Read, ReadSignal, Set, Update, WriteSignal
    },
    task::spawn_local,
    *,
};
use leptos_router::hooks::use_params;
use std::sync::{ Arc };
use leptos::prelude::Get;

use leptos_router::params::Params;
use crate::projects::{mock_content::get_project_content, model::{Project, ProjectDatabase}};


#[derive(Clone)]
pub struct ProjectContext {
    pub projects: (ReadSignal<Vec<Project>>, WriteSignal<Vec<Project>>),
    pub current_project_id: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub hovered_project_id: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub content: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    url_path: String,
}

impl ProjectContext {
    pub fn new() -> Self {
        let db = ProjectDatabase::new();
        let projects =db.get_all_projects();
        Self {
            projects: signal::<Vec<Project>>(projects.to_vec()),
            current_project_id: signal::<Option<String>>(None),
            hovered_project_id: signal::<Option<String>>(None),
            content: signal::<Option<String>>(None),
            is_loading: signal(false),
            error: signal(None),
            url_path: format!("projects"),
        }
    }

    pub fn fetch_project_content(&self, project_id: &str) {
        if let Ok(id) = project_id.parse::<u32>() {
            let content = get_project_content(id);
            // Fetch the project content from the database or API
            // Update the content signal with the fetched content
            self.content.1.set(content);
        } else {
            self.error.1.set(Some("Invalid project ID".to_string()));
        }
    }

    pub fn get_project_by_id(&self, project_id: &str) -> Option<Project> {
        if let Ok(id) = project_id.parse::<u32>() {
            self.projects.0.get().iter().find(|p| p.id == id).cloned()
        } else {
            None
        }
    }

    pub fn get_current_project(&self) -> Option<Project> {
        if let Some(id) = self.current_project_id.0.get().clone() {
            self.get_project_by_id(&id)
        } else {
            None
        }
    }
}

#[component]
pub fn ProjectProvider(children: Children) -> impl IntoView {
    let project_context = Arc::new(ProjectContext::new());
    provide_context(project_context);
    children()
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ProjectURLParams {
    pub project_id: Option<String>,
}



#[component]
pub fn ProjectRoute(children: Children) -> impl IntoView {

    let params = use_params::<ProjectURLParams>();
    let project_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.project_id.clone())
            .unwrap_or_default()
    };
    let project_context = use_project();
    Effect::new(move || {
        project_context.current_project_id.1.set(Some(project_id()));
        project_context.fetch_project_content(&project_id());
    });

    // Check projectentication status on initial load

    children()
}

pub fn use_project() -> Arc<ProjectContext> {
    use_context::<Arc<ProjectContext>>().expect(
        "ProjectContext not found. Make sure you are using ProjectProvider."
    )
}
