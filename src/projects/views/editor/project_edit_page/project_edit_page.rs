use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::projects::{
    projects_context::use_project, views::editor::project_edit_page::project_form::ProjectForm,
};

#[component]
pub fn ProjectEditPage() -> impl IntoView {
    let params = use_params_map();
    let project_context = use_project();

    // Get project ID from URL parameters
    let project_id = move || {
        params.with(|p| {
            p.get("project_id")
                .and_then(|id_str| {
                    if id_str == "new" {
                        None // Indicate we're creating a new project
                    } else {
                        id_str.parse::<i64>().ok()
                    }
                })
        })
    };

    // Check if we're in "new project" mode
    let is_new_project = move || {
        params.with(|p| {
            p.get("project_id").map(|id_str| id_str == "new").unwrap_or(false)
        })
    };

    // Find the specific project
    let project = move || {
        if let Some(id) = project_id() {
            project_context.projects.0.with(|projects| {
                projects.iter().find(|p| p.id == id as i32).cloned()
            })
        } else {
            None
        }
    };

    view! {
        //  <AreasEditor />
        <div class="min-h-screen grow">
            {move || {
                let is_creating_new = is_new_project();
                let existing_project = project();

                // Show form for both new projects and existing projects
                if is_creating_new || existing_project.is_some() {

                    view! {
                            // <h1 class="text-2xl font-bold mb-6 text-gray-800">{page_title}</h1>
                            {
                                if let Some(proj) = existing_project {
                                    view! {
                                        <ProjectForm
                                            project=proj
                                        />
                                        
                                    }.into_any()
                                } else {
                                    view! {
                                        <ProjectForm
                                        />
                                    }.into_any()
                                }
                            }
                    }.into_any()
                } else {
                    // Project not found case (only when we have a specific ID that doesn't exist)
                    view! {
                        <div class="text-center py-8">
                            <p class="text-gray-600">"Project not found"</p>
                            <a 
                                class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                                href="/editor"
                            >
                                "Back to Projects"
                            </a>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}