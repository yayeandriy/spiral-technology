use leptos::prelude::*;

use crate::{content::views::content_view::ContentView, projects::{projects_context::use_project, views::landing::{project_areas::ProjectAreas, project_header::ProjectHeader}}, ui::CloseButton};




#[component]
pub fn ProjectView() -> impl IntoView {
    let project_context = use_project();
    let (project_signal, set_project_signal) = signal(project_context.get_current_project());
    
    // Update the signal when project context changes
    Effect::new(move |_| {
        set_project_signal.set(project_context.get_current_project());
    });
    
    view! {
        <div class="fixed top-0 left-[700px]">
        <div class="h-screen relative  w-[800px]   overflow-y-auto p-8 transition-all">
        <div class=" flex w-full flex-col border h-full overflow-y-auto rounded-lg bg-white shadow-md">
            <ProjectAreas project=project_signal />
            <ProjectHeader project=project_signal />
            <ContentView />
        </div>
        <div class="absolute top-2 right-2">
            <CloseButton href="/home".to_string() />
        </div>
        </div>
        </div>
    }
}