use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::projects::views::{landing::projects_list_view::ProjectsView};





#[component]
pub fn HomePage() -> impl IntoView {
  
   
    view! {
        <main class="w-screen h-screen flex justify-start bg-white  items-start text-black " style="line-height: 1.5;">
                <div class="flex flex-col space-y-2 pt-[80px] mr-24 ml-24 w-[200px]  sticky top-8 bg-white z-10 ">
                    <a href="/about" class="w-[100px] ">
                        <img class="w-24 h-24" src="/public/logo-black@2x.svg" alt="logo" />               
                    </a>
                    <div >
                        <div class=" pb-8 pt-8">
                        <b>"Spiral"</b><br/> 
                        <i class="text-[15px]">"Science & Technology ltd."</i>
                        </div>
                        <div class="text-gray-600 mt-4 flex flex-col space-y-3">
                            <div class="flex justify-between items-end text-[12px]" >
                                <div class="uppercase tracking-wider  font-semibold">
                                    Projects
                                </div>
                                <div class="grow border-b border-gray-300 border-dotted" />
                                <div class="font-mono" >
                                    18
                                </div>
                            </div>
                            <div class="flex justify-between items-end text-[12px]" >
                                <div class="uppercase tracking-wider  font-semibold">
                                    POC
                                </div>
                                <div class="grow border-b border-gray-300 border-dotted" />
                                <div class="font-mono" >
                                    12
                                </div>
                            </div>
                            <div class="flex justify-between items-end text-[12px]" >
                                <div class="uppercase tracking-wider  font-semibold">
                                    Pilots
                                </div>
                                <div class="grow border-b border-gray-300 border-dotted" />
                                <div class="font-mono" >
                                    4
                                </div>
                            </div>
                            <div class="flex justify-between items-end text-[12px]" >
                                <div class="uppercase tracking-wider  font-semibold">
                                    Clients
                                </div>
                                <div class="grow border-b border-gray-300 border-dotted" />
                                <div class="font-mono" >
                                    8
                                </div>
                            </div>

                
                        </div>
                    </div>
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
