use leptos::prelude::*;

use crate::{content::{self, content_context::use_project_content}, projects::{model::Project, projects_context::use_project}};




#[component]
pub fn ProjectHeader(
    project: Project
) -> impl IntoView {
    let project_clone = project.clone();
    view! {
        <div class="w-full flex flex-col border-t pt-4 px-4">
            <span class="bg-white pr-2">{project.title}</span>           
            <div class="text-gray-400 h-32px" >{project_clone.desc}</div>
        </div>
    
    }
}