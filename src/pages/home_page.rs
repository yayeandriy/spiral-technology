use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::projects::views::{landing::projects_list_view::ProjectsView};





#[component]
pub fn HomePage() -> impl IntoView {
  
   
    view! {
        <main class="w-screen h-screen flex justify-start bg-white relative items-start text-black " style="line-height: 1.5;">
                <div class="flex flex-col space-y-2 pt-[80px] pl-8 pr-10 pb-10 mx-10 w-[280px] 
                hover:shadow-xl hover:w-[800px] bg-white z-[200] cursor-pointer
                transition-all duration-[0.2s] ease-out fixed mt-20 hover:border  ">
                <div class="w-[260px] " >
                    <a href="/about" class="w-[100px] ">
                        <img class="w-24 h-24" src="/public/logo-black@2x.svg" alt="logo" />               
                    </a>
                    <div >
                        <div class=" pb-8 pt-8">
                        <b>"Spiral"</b><br/> 
                        <i class="text-[15px]">"Science & Technology ltd."</i>
                        </div>
                        <div class="text-[15px]">
                            "We are dedicated to advancing industrial inspection and monitoring through innovative, technology-driven solutions. The company specializes in precision systems for sectors such as aerospace, energy, and manufacturing, delivering capabilities that range from turbine and blade inspection to assembly line quality control and robotic visual guidance."
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
                </div>
                <div class="ml-[460px] ">
                    <ProjectsView />
                </div> 
                <div class="h-screen w-[1000px] pr-16 ml-[0px]" >
                    <Outlet />
                </div>
        </main>
    }
}
