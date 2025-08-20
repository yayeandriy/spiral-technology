use leptos::{logging, prelude::*};
use web_sys::MouseEvent;

use crate::projects::{model::Project, projects_context::use_project};




#[component]
pub fn ProjectsView() -> impl IntoView {
    let project_context = use_project();
    let project_context_clone = use_project();
    let project_context_clone_2 = use_project();
    let project_context_clone_3 = use_project();
    let current_project_id = move || project_context_clone_3.current_project_id.0.get();
    let projects = move || project_context.projects.0.get();
    let handle_project_mouseenter = move |project: Project| {
        logging::log!("Project Hovered: {}", project.id);
        // Handle project hover logic here
        project_context_clone.hovered_project_id.1.set(Some(project.id.to_string()));
    };
    let handle_project_mouseleave = move || {
        // Handle project hover logic here
        project_context_clone_2.hovered_project_id.1.set(None);
    };
    view! {
        <div class="flex flex-col gap-2 mt-[42px] pb-20 ">
        <div class="text-gray-400 sticky top-0 bg-white z-10    " >
            Projects
        </div>
            {
                move || { 
                     let mut projects_vec = projects();
                    projects_vec.sort_by(|a, b| a.title.cmp(&b.title));
                    
                    projects_vec.into_iter()                    
                    .map(|project| {
                    let project_clone = project.clone();
                    let handle_project_mouseenter = handle_project_mouseenter.clone();
                    let handle_project_mouseleave = handle_project_mouseleave.clone();
                    let is_project_current = current_project_id() == Some(project_clone.id.to_string());
                    let title_class = if is_project_current {
                        "text-blue-500 "
                    } else {
                        "text-black"
                    };
                    view! {
                        <div 
                         on:mouseenter=move |_:MouseEvent| {
                            let project_clone = project_clone.clone();
                            handle_project_mouseenter(project_clone);
                        }
                         on:mouseleave=move |_:MouseEvent| {
                            handle_project_mouseleave();
                        }
                        >
                        <a 
                        href={format!("/home/{}", project.id)}
                        
                        class="cursor-pointer h-[72px] flex group flex-col transition-colors duration-200 hover:text-black text-gray-800">
                            <div
                           
                                class="h-[32px] " 
                        style="background: linear-gradient(
                                  to bottom,
                                  transparent 49%,   
                                  #dfdfdf 49%,         
                                  #dfdfdf 51%,         
                                  transparent 51%    
                                )"
                            >
                                <span class=format!("bg-white w-[200px] truncate pr-2 group-hover:text-blue-500 {title_class}")>{project.title}</span>
                            </div>
                            <div class="text-gray-400 h-32px" >{project.desc}</div>
                        </a>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
            }
        </div>
    }
}