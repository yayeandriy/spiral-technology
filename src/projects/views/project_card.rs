use leptos::prelude::*;

use crate::{projects::model::Project, ui::form};
#[component]
pub fn ProjectsCard(
    project: Project,
    is_selected: bool,
) -> impl IntoView {
          let base_class = "p-2 text-sm first:rounded-t-[4px] last:rounded-b-[4px] first:border-t border-x border-b ";
          let selected_class = if is_selected { "bg-blue-500 hover:bg-blue-600 text-white border-blue-500" } else { " hover:bg-gray-50 text-black border-gray-200" };
        view! {
            <div class=format!("{} {}", base_class, selected_class)>
                 <a class="flex flex-col items-start" href=format!("/editor/{}", project.id) >
                       <div>
                            {project.clone().title} 
                       </div>                        
                       <div class="opacity-60 h-6">
                            {project.clone().desc} 
                       </div>                        
                    </a>               
            </div>
        }                        
}