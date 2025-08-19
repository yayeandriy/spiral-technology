use leptos::prelude::*;

use crate::projects::{projects_context::use_project};




#[component]
pub fn ProjectsView() -> impl IntoView {
    let project_context = use_project();
    let projects = move || project_context.projects.0.get();

    view! {
        <div class="flex flex-col gap-2 mt-20">
            {
                move || projects().into_iter().map(|project| {
                    view! {
                        <a 
                        href={format!("/home/{}", project.id)}
                        class="cursor-pointer h-[72px] flex flex-col transition-colors duration-200 hover:text-black text-gray-800">
                            <div
                                class="h-[32px]" 
                        style="background: linear-gradient(
                                  to bottom,
                                  transparent 49%,   
                                  #dfdfdf 49%,         
                                  #dfdfdf 51%,         
                                  transparent 51%    
                                )"
                            >
                                <span class="bg-white pr-2">{project.title}</span>
                            </div>
                            <div class="text-gray-400 h-32px" >{project.desc}</div>
                        </a>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}