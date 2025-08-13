use leptos::prelude::*;

use crate::projects::projects_context::use_project;


#[component]
pub fn ProjectPage() -> impl IntoView {
    let project_context = use_project();
    let cloned_context = project_context.clone();
    let content = move || {
        cloned_context.content.0.get().clone()
    };

    let project = move || {
        project_context.get_current_project()
    };

    view! {
        <div class="flex flex-col p-4 gap-4 w-[600px]" >
            <a href="/">
                "Back"
            </a>
            <div>
               {move || {
                if let Some(project) = project() {
                    view!{
                        <h1>{project.title.clone()}</h1>
                    }.into_any()
                } else {
                    // Handle case where project is not found
                    view!{
                        <h1>"Project not found"</h1>
                    }.into_any()
                }
               }}
            </div>
            <div>
                {move || content()}
            </div>
        </div>
    }
}
