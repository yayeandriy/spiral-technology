use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::projects::{
    model::Project, 
    projects_context::use_project,
    views::projects_list::ProjectsList
};

#[component]
pub fn ProjectsEditor() -> impl IntoView {
    let project_context = use_project();
    let navigate = use_navigate();
    
    let navigate_create = navigate.clone();
    let navigate_edit = navigate.clone();
    
    // Handle creating a new project - navigate to the create URL
    let handle_create = move |_: ()| {
        navigate_create("/editor/new", Default::default());
    };
    
    // Handle editing an existing project - navigate to the edit URL
    let handle_edit = move |project: Project| {
        let project_id = project.id;
        navigate_edit(&format!("/editor/{}", project_id), Default::default());
    };
    
    // Handle deleting a project
    let handle_delete = move |project_id: i32| {
        let context = project_context.clone();
        spawn_local(async move {
            context.delete_project(project_id).await;
        });
    };

    view! {
        <div class="container mx-auto p-4 text-black">
            <ProjectsList 
                on_create=Callback::new(handle_create)
                on_edit=Callback::new(handle_edit)
                on_delete=Callback::new(handle_delete)
            />
        </div>
    }
}