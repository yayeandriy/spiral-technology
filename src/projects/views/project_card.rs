use leptos::prelude::*;

use crate::projects::model::Project;
#[component]
pub fn ProjectsCard(
    project: Project,
) -> impl IntoView {
        view! {
            <div class="p-2 text-sm first:rounded-t-[4px] last:rounded-b-[4px] first:border-t border-x border-b border-gray-200  hover:bg-gray-50">
                 <a class="flex flex-col items-start" href=format!("/editor/{}", project.id) >
                       <div>
                            {project.clone().title} 
                       </div>                        
                       <div class="text-gray-400 h-6">
                            {project.clone().desc} 
                       </div>                        
                    </a>               
            </div>
        }                        
}