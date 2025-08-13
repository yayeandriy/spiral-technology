use leptos::prelude::*;
use leptos_router::components::Outlet;
use web_sys::MouseEvent;

use crate::projects::projects_context::use_project;



#[component]
pub fn HomePage() -> impl IntoView {
    let project_context = use_project();
    let project_context_clone = project_context.clone();
    let projects = move || project_context.projects.0.get();
    let _is_item_selected = signal(true);
    let hover_handler = move |id: String| {
        project_context_clone.hovered_project_id.1.set(Some(id));
    };
   
    view! {
        <main class="w-screen h-screen flex justify-start bg-white leading-[1.01] items-start text-black gap-4 p-8 text-[20px] pl-[100px] pt-16" style="line-height: 1.5;">
            <div class="flex flex-col gap-2 pt-[80px] ">
                <a href="/about">
                    <img class="w-24 h-24" src="/public/logo-black@2x.svg" alt="logo" />               
                </a>
            </div>
            <div class="flex" >
                <div class="flex flex-col gap-y-2 pl-24 pt-8">
                    <ol class="list-decimal transition-all text-gray-500 hover:marker:text-zinc-800 cursor-pointer marker:text-zinc-200 marker:font-mono marker:font-normal">
                    {
                        move || projects().iter().map(|project| {
                            let project = project.clone();
                            let hover_handler = hover_handler.clone();
                            view! {
                                
                                    <li class="pl-2 mb-2 text-black"
                                    on:mouseenter=move |_: MouseEvent| {
                                        let project_id = project.id.clone().to_string();
                                        hover_handler(project_id.clone())
                                    }>
                                       <a href={format!("/{}", project.id)}>
                                           <div>
                                           {project.title.clone()}</div>
                                        //    {
                                        //     move || project.technologies.iter().map(|tech| {
                                        //         view! {
                                        //             <span class="text-gray-500 font-mono text-[9px] uppercase">{tech.to_string()}</span>
                                        //         }
                                        //     }).collect_view()
                                        //    }
                                       </a>
                                    </li>
                            }
                        }).collect::<Vec<_>>()
                }
                </ol>
                </div> 
               </div> 
               <div class="grow" >
                <Outlet />
               </div>
        </main>
    }
}
