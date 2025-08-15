use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{areas::views::areas_editor::AreasEditor, projects::views::projects_editor::ProjectsEditor};


#[component]
pub fn EditorPage() -> impl IntoView {
    view! {
        <main class="w-full h-screen bg-white flex items-start justify-start p-8 text-[20px]" style="line-height: 1.5;">
        <ProjectsEditor />
        <AreasEditor />
         <div class="flex w-full">         
            <Outlet />
         </div> 
        </main>
    }
}
