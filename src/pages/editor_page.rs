use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::projects::views::editor::projects_list::ProjectsList;



#[component]
pub fn EditorPage() -> impl IntoView {
    view! {
        <main class="w-screen h-screen bg-white flex items-start justify-start p-8 text-[20px]" style="line-height: 1.5;">
            <a href="/home" class="m-4 sticky top-16">
                <img class="w-24 h-24" src="/public/logo-black@2x.svg" />
            </a>
            <ProjectsList />
            <Outlet />
        </main>
    }
}
