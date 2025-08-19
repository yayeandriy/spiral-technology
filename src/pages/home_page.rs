use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::projects::views::{landing::projects_list_view::ProjectsView};





#[component]
pub fn HomePage() -> impl IntoView {
  
   
    view! {
        <main class="w-screen h-screen overflow-y-hidden flex justify-start bg-white leading-[1.01] items-start text-black text-[20px] pt-8" style="line-height: 1.5;">
                <div class="flex flex-col gap-2 pt-[80px] mr-8 ml-20">
                    <a href="/about">
                        <img class="w-24 h-24" src="/public/logo-black@2x.svg" alt="logo" />               
                    </a>
                </div>
                <div>
                    <ProjectsView />
                </div> 
                <div class="overflow-y-hidden h-screen w-[800px]" >
                    <Outlet />
                </div>
        </main>
    }
}
