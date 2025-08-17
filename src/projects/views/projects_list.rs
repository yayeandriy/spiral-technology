use leptos::prelude::*;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::projects::views::project_card::ProjectsCard;
use crate::ui::signal_button::SPrimaryButton;
use crate::ui::*;

#[component]
pub fn ProjectsList(
    #[prop(optional)] on_create: Option<Callback<()>>,
) -> impl IntoView {
    let project_context = use_project();
    let cloned_context = project_context.clone();
    let projects = move || cloned_context.projects.0.get(); 
    let handle_create = move |_| {
        if let Some(create_callback) = on_create {
            create_callback.run(());
        }
    };

    view! {
        <div class="flex flex-col p-4 gap-4 w-[400px]" >
            <div class="flex justify-between items-center">
                <SPrimaryButton on_click=handle_create >
                    "Create New Project"
                </SPrimaryButton>
            </div>
            
            <div class="">
                {move || {
                    let project_list = projects();
                    if project_list.is_empty() {
                        view! {
                            <div class="text-gray-500 text-center py-8">
                                "No projects found."
                            </div>
                        }.into_any()
                    } else {
                        project_list.into_iter().map(|project| {
                            view! {
                               <ProjectsCard project=project />
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>
        </div>
    }
}