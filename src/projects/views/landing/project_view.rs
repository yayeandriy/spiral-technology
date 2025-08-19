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
        <div class="h-screen w-full overflow-y-hidden p-8 transition-all">
        <div class="relative flex w-full flex-col border h-full overflow-y-auto rounded-lg bg-white shadow-xl">

           <ProjectAreas project=project_signal />
            <ProjectHeader project=project_signal />
            <ContentView />
            <CloseButton href="/home".to_string() />
        </div>
        </div>
    }
}