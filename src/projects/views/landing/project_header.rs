use leptos::{html::Div, prelude::*};
use leptos_use::use_element_visibility;

use crate::projects::model::Project;




#[component]
pub fn ProjectHeader(
    project: ReadSignal<Option<Project>>
) -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let is_visible = use_element_visibility(el);

   
    let base_class = "w-full sticky bg-white transition-all flex flex-col border-t pt-4 pb-4 px-4";
    let div_class = move || {
        let visible = is_visible.get();
        if visible {
            format!("{}  ", base_class)
        } else {
            format!("{} top-[60px] ", base_class)
        }
        
    };
    view! {
        <div node_ref=el />
        <div class=div_class>
            {
                move || if let Some(proj) = project.get() {
                    view! {
                        <>
                            <span class="pr-2">{proj.title}</span>           
                            <div class="text-gray-400 h-32px">{proj.desc}</div>
                        </>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-gray-400 italic">
                            "No project selected"
                        </div>
                    }.into_any()
                }
            }
        </div>
         
    }
}