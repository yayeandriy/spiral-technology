use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::projects::views::{landing::projects_list_view::ProjectsView};





#[component]
pub fn HomePage() -> impl IntoView {
  
   
    view! {
        <main class="w-screen h-screen flex justify-start bg-white  items-start text-black pt-8" style="line-height: 1.5;">
                <div class="flex flex-col gap-2 pt-[80px] mr-8 ml-20 w-[100px]  sticky top-8 bg-white z-10 ">
                    <a href="/about" class="w-[100px] ">
                        <img class="w-24 h-24" src="/public/logo-black@2x.svg" alt="logo" />               
                    </a>
                </div>
                <div>
                    <ProjectsView />
                </div> 
                <div class="h-screen w-[1000px] pr-8" >
                    <Outlet />
                </div>
        </main>
    }
}
