use crate::{content::model::ProjectContent, projects::projects_context::ProjectURLParams, supabase::{supabase_delete, supabase_get, supabase_patch, supabase_post}};
use leptos::{
    logging,
    prelude::{
        provide_context, signal, use_context, Children, Effect, Read, ReadSignal, Set, Update, WriteSignal
    },
    task::spawn_local,
    *,
};
use leptos_router::hooks::use_params;
use std::sync::Arc;
use leptos::prelude::Get;



#[derive(Clone)]
pub struct ProjectContentContext {
    pub project_content: (ReadSignal<Option<ProjectContent>>, WriteSignal<Option<ProjectContent>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub project_id: (ReadSignal<Option<i64>>, WriteSignal<Option<i64>>),
}

impl ProjectContentContext {
    pub fn new() -> Self {
        Self {
            project_content: signal::<Option<ProjectContent>>(None),
            is_loading: signal(false),
            error: signal(None),
            project_id: signal(None),
        }
    }

    pub fn set_project_id(&self, id: i64) {
        self.project_id.1.set(Some(id));
    }

    pub async fn fetch_project_content(&self) {
        let project_id = match self.project_id.0.get() {
            Some(id) => id,
            None => {
                logging::log!("No project ID set");
                self.error.1.set(Some("No project ID set".to_string()));
                return;
            }
        };

        self.is_loading.1.try_update(|v| {
            *v = true;
        });
        self.error.1.update(|e| {
            *e = None;
        });
        
        let url_path = format!("/rest/v1/content?project_id=eq.{}&select=*", project_id);
        match supabase_get::<Vec<ProjectContent>>(&url_path).await {
            Ok(items) => {
                logging::log!("Fetched project content successfully: {:?}", items);
                // Get the first (and should be only) project content for this project
                let content = items.into_iter().next();
                self.project_content.1.set(content);
            }
            Err(err) => {
                logging::log!("Error fetching project content: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }

    pub async fn create_or_update_project_content(&self, text: Option<String>) {
        let project_id = match self.project_id.0.get() {
            Some(id) => id,
            None => {
                logging::log!("No project ID set");
                self.error.1.set(Some("No project ID set".to_string()));
                return;
            }
        };

        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);

        let current_content = self.project_content.0.get();
        
        match current_content {
            Some(existing_content) => {
                // Update existing content
                let updated_content = serde_json::json!({
                    "text": text,            
                });
                
                let url_path = format!("/rest/v1/content?id=eq.{}", existing_content.id);
                match supabase_patch::<ProjectContent, serde_json::Value>(&url_path, &updated_content).await {
                    Ok(updated) => {
                        self.project_content.1.set(Some(updated));
                    }
                    Err(err) => {
                        logging::log!("Error updating project content: {}", err);
                        self.error.1.set(Some(err));
                    }
                }
            }
            None => {
                // Create new content
                let new_content = serde_json::json!({
                    "text": text,
                    "project_id": project_id
                });
                
                match supabase_post::<ProjectContent, serde_json::Value>("/rest/v1/content", &new_content).await {
                    Ok(content) => {
                        self.project_content.1.set(Some(content));
                    }
                    Err(err) => {
                        logging::log!("Error creating project content: {}", err);
                        self.error.1.set(Some(err));
                    }
                }
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }

    pub async fn delete_project_content(&self) {
        let content = match self.project_content.0.get() {
            Some(content) => content,
            None => {
                logging::log!("No project content to delete");
                return;
            }
        };

        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        
        let url_path = format!("/rest/v1/content?id=eq.{}", content.id);
        match supabase_delete(&url_path).await {
            Ok(_) => {
                self.project_content.1.set(None);
            }
            Err(err) => {
                logging::log!("Error deleting project content: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }
}#[component]
pub fn ProjectContentContextProvider(children: Children) -> impl IntoView {
    let content_context = Arc::new(ProjectContentContext::new());
   
    provide_context(content_context);

    children()
}

#[component]
pub fn ProjectContentRoute( children: Children) -> impl IntoView {
    let params = use_params::<ProjectURLParams>();
    let project_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.project_id.clone())
            .unwrap_or_default()
    };
    let project_content_context = use_project_content();
    if let Ok(id) = project_id().parse::<i64>() {
        project_content_context.set_project_id(id);
        
        spawn_local(async move {
            project_content_context.fetch_project_content().await;
        });
    }

    children()
}


pub fn use_project_content() -> Arc<ProjectContentContext> {
    use_context::<Arc<ProjectContentContext>>().expect(
        "ProjectContentContext not found. Make sure you are using ProjectContentContextProvider."
    )
}
