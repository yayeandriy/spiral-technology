use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::projects::projects_context::use_project;
use crate::projects::views::editor::project_card::ProjectsCard;
use crate::ui::button::PrimaryButton;

#[component]
pub fn ProjectsList(
) -> impl IntoView {
    let project_context = use_project();
    let cloned_context = project_context.clone();
    let current_project_id = move || cloned_context.current_project_id.0.get();
    let cloned_context = project_context.clone();
    let projects = move || cloned_context.projects.0.get(); 

    let navigate = use_navigate();
    let navigate_create = navigate.clone();
    
    // Handle creating a new project - navigate to the create URL
    let handle_create = move |_| {
        navigate_create("/editor/new", Default::default());
    };



    view! {
        <div class="flex flex-col p-4 gap-4 w-[400px]" >
            <div class="flex justify-between items-center">
                <PrimaryButton on_click=handle_create >
                    "Create new project"
                </PrimaryButton>
            </div>
            
            <div class="">
                {move || {
                    let mut project_list = projects();
                    project_list.sort_by_key(|p| p.title.clone());

                    if project_list.is_empty() {
                        view! {
                            <div class="text-gray-500 text-center py-8">
                                "No projects found."
                            </div>
                        }.into_any()
                    } else {
                        project_list.into_iter().map(|project| {
                            let is_project_selected = current_project_id() == Some(project.id.to_string());
                            view! {
                               <ProjectsCard project=project is_selected=is_project_selected />
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>
        </div>
    }
}