use leptos::prelude::*;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;

#[component]
pub fn ProjectsList(
    #[prop(optional)] on_edit: Option<Callback<Project>>,
    #[prop(optional)] on_create: Option<Callback<()>>,
    #[prop(optional)] on_delete: Option<Callback<i32>>,
) -> impl IntoView {
    let project_context = use_project();
    let cloned_context = project_context.clone();
    let projects = move || cloned_context.projects.0.get(); 

    view! {
        <div class="flex flex-col p-4 gap-4 w-[600px]" >
            <div class="flex justify-between items-center">
                <h2 class="text-xl font-semibold">"Projects"</h2>
                {move || {
                    if let Some(create_callback) = on_create {
                        view! {
                            <button
                                on:click=move |_| create_callback.run(())
                                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                            >
                                "Create New Project"
                            </button>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>
            
            <div class="space-y-2">
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
                            let project_for_edit = project.clone();
                            let project_for_delete = project.clone();
                            view! {
                                <div class="p-4 border border-gray-200 rounded-lg hover:bg-gray-50">
                                    <div class="flex justify-between items-start">
                                        <div class="flex-1">
                                            <h3 class="font-medium">{project.title.clone()}</h3>
                                            {project.desc.clone().map(|desc| view! {
                                                <p class="text-sm text-gray-600 mt-1">{desc}</p>
                                            })}
                                        </div>
                                        <div class="flex gap-2">
                                            {move || {
                                                if let Some(edit_callback) = on_edit {
                                                    let project_clone = project_for_edit.clone();
                                                    view! {
                                                        <button
                                                            on:click=move |_| edit_callback.run(project_clone.clone())
                                                            class="px-3 py-1 text-sm bg-blue-100 text-blue-700 rounded hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                        >
                                                            "Edit"
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }
                                            }}
                                            {move || {
                                                if let Some(delete_callback) = on_delete {
                                                    let project_id = project_for_delete.id;
                                                    let project_title = project_for_delete.title.clone();
                                                    view! {
                                                        <button
                                                            on:click=move |_| {
                                                                if web_sys::window()
                                                                    .unwrap()
                                                                    .confirm_with_message(&format!("Are you sure you want to delete '{}'?", project_title))
                                                                    .unwrap_or(false)
                                                                {
                                                                    delete_callback.run(project_id);
                                                                }
                                                            }
                                                            class="px-3 py-1 text-sm bg-red-100 text-red-700 rounded hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-red-500"
                                                        >
                                                            "Delete"
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }
                                            }}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </div>
        </div>
    }
}