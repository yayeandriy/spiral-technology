use leptos::prelude::*;

use crate::{content::{content_context::use_project_content, views::content_view::ContentView}, projects::{projects_context::use_project, views::landing::{project_areas::ProjectAreas, project_header::ProjectHeader}}};




#[component]
pub fn ProjectView() -> impl IntoView {
    let project_context = use_project();
    let (project_signal, set_project_signal) = signal(project_context.get_current_project());
    
    // Update the signal when project context changes
    Effect::new(move |_| {
        set_project_signal.set(project_context.get_current_project());
    });
    
    let content_context = use_project_content();
    let content = move || content_context.project_content.0.get();
    
    view! {
        <div class="h-screen w-full overflow-y-hidden p-8 transition-all">
        <div class="relative flex w-full flex-col border h-full overflow-y-auto rounded-lg bg-white shadow-xl">

           <ProjectAreas project=project_signal />
            {
                move || if let Some(project) = project_signal.get() {
                    view! {
                        <ProjectHeader project=project />
                    }.into_any()
                }else{
                    view!{<div/>}.into_any()
                }
            }
            {
                move || if let Some(content) = content() {
                    view! {
                        <ContentView content=content />
                    }.into_any()
                }else{
                    view!{<div/>}.into_any()
                }
            }
            <a href="/home"             
            class="w-12 h-12 absolute top-2 right-2 text-lg  rounded-full bg-gray-100 flex items-center justify-center justify-self-end cursor-pointer hover:bg-gray-200">
                <div>
                    "╳"
                </div>
            </a>
        </div>
        </div>
    }
}