
use leptos::prelude::*;

use crate::{areas::areas_context::use_areas, catalog::catalog_context::use_catalog, projects::projects_context::use_project};





#[component]
pub fn AreasTable() -> impl IntoView {

    let areas_context = use_areas();
    let project_context = use_project();
    let projects = move || project_context.projects.0.get();
    let catalog_context = use_catalog();
    let default_category = String::from("technologies");
    let current_category = signal::<String>(default_category);

    let areas_context_clone = areas_context.clone();
    let areas = move || areas_context.get_area_by_category(&current_category.0.get());
    let areas_clone = areas.clone();
    let categories = move || areas_context_clone.categories.0.get();

    let project_line = move |ids: Vec<i64>, project_index: usize| {
        view!{
         <div class="flex w-full justify-between">
            {
                areas_clone().iter().map(|area| {
                    let area_clone = area.clone();
                    let is_area_in_project = ids.contains(&area_clone.id);
                    let ml = if is_area_in_project { "" } else { "-ml-96" };
                    let delay = 0.05 * project_index as f32;
                    view! {
                       <div class="cursor-pointer  h-[72px] w-full flex flex-col transition-colors duration-200 hover:text-black text-gray-800">
                            <div
                                class="h-[32px]  overflow-x-hidden w-full flex justify-center items-center" 
                        style="background: linear-gradient(
                                  to bottom,
                                  transparent 49%,   
                                  #dfdfdf 49%,         
                                  #dfdfdf 51%,         
                                  transparent 51%    
                                )"
                            >
                                <div 
                                class=format!("h-2 w-2 {ml} relative bg-black rounded-full transition-all ease-out duration-[1s]")  
                                style=format!("transition-delay: {delay}s;")
                                />
                            </div>
                            <div class="text-gray-400 h-32px" ></div>
                        </div>
                    }
                }).collect_view()
            }
         </div>   
        }
    };

    let areas_project_match = move ||{
        view!{
            <div class="flex flex-col w-full gap-2 mt-3">
                {
                    projects().iter().enumerate().map(|(project_index, project)| {
                        let project_clone = project.clone();
                        let areas_ids = catalog_context.get_project_areas_ids(project_clone.id as i64);
                        project_line(areas_ids, project_index)
                    }).collect_view()
                }
            </div>
        }
    };
    view! {
        <div class="flex w-full justify-between mb-2">
            {
                move || categories().into_iter().map(|cat| {
                    let cat_clone = cat.clone();
                    let is_current = current_category.0.get() == cat_clone;
                    view! {
                        <div
                            on:click=move |_| {
                                current_category.1.set(cat_clone.clone());
                            }
                            class="cursor-pointer transition-colors duration-200"
                            class:text-gray-200={!is_current}
                            >
                            <div>{cat}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
        <div class="flex w-full justify-between">
            {
                move || areas().into_iter().map(|area| {
                    view! {
                        <div class="w-full text-center">
                            <div class="">{area.title}</div>
                            // <div class="text-sm text-gray-500">{area.desc.clone().unwrap_or_default()}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
        <div>
            {move || areas_project_match()}
        </div>
    }
}
