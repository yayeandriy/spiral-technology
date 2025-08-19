use leptos::prelude::*;

use crate::projects::model::Project;




#[component]
pub fn ProjectHeader(
    project: ReadSignal<Option<Project>>
) -> impl IntoView {
    view! {
        <div class="w-full sticky bg-white top-[200px] flex flex-col border-t pt-4 px-4">
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