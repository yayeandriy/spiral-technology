use leptos::prelude::*;

use crate::{content::{self, content_context::use_project_content}, projects::{model::Project, projects_context::use_project}};




#[component]
pub fn ProjectHeader(
    project: Project
) -> impl IntoView {
    
    view! {
        <div class="cursor-pointer h-[72px] flex flex-col transition-colors duration-200 hover:text-black text-gray-800">
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
        </div>
    
    }
}