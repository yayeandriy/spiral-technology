use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::projects::views::projects_list::ProjectsList;





#[component]
pub fn HomePage() -> impl IntoView {
  
   
    view! {
        <main class="w-screen h-screen flex justify-start bg-white leading-[1.01] items-start text-black gap-4 p-8 text-[20px] pl-[100px] pt-16" style="line-height: 1.5;">
                <div>
                    <ProjectsList />
                </div> 
               <div class="grow" >
                <Outlet />
               </div>
        </main>
    }
}
