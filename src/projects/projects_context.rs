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
use crate::{projects::model::{Project, ProjectDto}, supabase::{supabase_delete, supabase_get, supabase_patch, supabase_post}};


#[derive(Clone)]
pub struct ProjectContext {
    pub projects: (ReadSignal<Vec<Project>>, WriteSignal<Vec<Project>>),
    pub current_project_id: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub hovered_project_id: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    url_path: String,
}

impl ProjectContext {
    pub fn new() -> Self {
        Self {
            projects: signal::<Vec<Project>>(vec![]),
            current_project_id: signal::<Option<String>>(None),
            hovered_project_id: signal::<Option<String>>(None),
            is_loading: signal(false),
            error: signal(None),
            // url_path: "/rest/v1/projects?select=*".to_string(),
            url_path: format!("/rest/v1/projects"),
        }
    }

    pub async fn fetch_projects(&self) {
        self.is_loading.1.try_update(|v| {
            *v = true;
        });
        self.error.1.update(|e| {
            *e = None;
        });
        let url = format!("{}?select=*", self.url_path);
        match supabase_get::<Vec<Project>>(&url).await {
            Ok(items) => {
                logging::log!("Fetched projects successfully: {:?}", items);
                self.projects.1.set(items);
            }
            Err(err) => {
                logging::log!("Error fetching projects: {}", err);
            }
        }   
    } 

    pub async fn add_project(&self, project: Project) -> Option<Project> {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        let new_project = ProjectDto {
            title: project.title,
            desc: project.desc,
            ..ProjectDto::default()
        };
        match supabase_post::<Project, ProjectDto>(&format!("{}", self.url_path),&new_project).await  {
            Ok(item) => {                         
                self.projects.1.update(|items| {
                    items.push(item.clone());
                });
                Some(item)
            }
            Err(err) => {
                logging::log!("Error creating project: {}", err);
                None
            }
        }
    }

    pub async fn update_project(&self, project: Project) {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        let project_dto = project.to_dto();
        let id = project.id.clone();
        match supabase_patch::<Project, ProjectDto>(&format!("{}?id=eq.{}", self.url_path, project.id),&project_dto).await  {
            Ok(item) => {                     
                self.projects.1.update(|items| {
                    items.iter_mut().for_each(|i| {
                        if i.id == id {
                            *i = item.clone();
                        }
                    });
                });
            }
            Err(err) => {
                logging::log!("Error fetching items: {}", err);
            }
        }
      
    }

    pub async fn delete_project(&self, project_id: i32) {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        
        match supabase_delete(&format!("{}?id=eq.{}", self.url_path, project_id)).await {
            Ok(_) => {
                self.projects.1.update(|items| {
                    items.retain(|item| item.id != project_id);
                });
            }
            Err(err) => {
                logging::log!("Error deleting project: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }


    pub fn get_project_by_id(&self, project_id: &str) -> Option<Project> {
        if let Ok(id) = project_id.parse::<i32>() {
            self.projects.0.get().iter().find(|p| p.id == id).cloned()
        } else {
            None
        }
    }

    pub fn get_current_project(&self) -> Option<Project> {
        self.current_project_id.0.get().as_ref().and_then(|id| self.get_project_by_id(id))
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
    let project_context_clone = project_context.clone();
    spawn_local(async move {        
        project_context.fetch_projects().await;       
    });

    Effect::new(move || {
        project_context_clone.current_project_id.1.set(Some(project_id()));
        // project_context.fetch_project_content(&project_id());
    });

    // Check projectentication status on initial load

    children()
}

pub fn use_project() -> Arc<ProjectContext> {
    use_context::<Arc<ProjectContext>>().expect(
        "ProjectContext not found. Make sure you are using ProjectProvider."
    )
}
