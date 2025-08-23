
use leptos::{logging, prelude::*};

use crate::{areas::areas_context::use_areas, catalog::catalog_context::use_catalog, projects::projects_context::use_project};





#[component]
pub fn AreasTable() -> impl IntoView {

    let areas_context = use_areas();
    let areas_context_clone = areas_context.clone();
    let project_context = use_project();
    let project_context_clone = project_context.clone();
    let projects = move || project_context.projects.0.get();
    let hovered_project_id = move || {
        let id = project_context_clone.hovered_project_id.0.get();
        if let Some(id) = id {
            logging::log!("Hovered Project ID: {}", id);
        }
        project_context_clone.hovered_project_id.0.get()
    };  
    let catalog_context = use_catalog();
    let default_category = move || areas_context_clone.default_category.0.get(); 
    let current_category = signal::<Option<String>>(None);
    Effect::new(move || {
        if let Some(default_cat) = default_category() {
            if current_category.0.get().is_none() {
                current_category.1.set(Some(default_cat));
            }
        } else {
            logging::log!("No default category found");
        }
    });

    let areas_context_clone = areas_context.clone();
    let areas = move || {
        if let Some(current_cat) = current_category.0.get() {
            areas_context.get_areas_by_category(&current_cat)
        } else {
            vec![]
        }
    };
    let areas_clone = areas.clone();
    let categories = move || areas_context_clone.categories.0.get();

    let project_line = move |ids: Vec<i64>, project_index: usize, is_project_hovered: bool| {
        view!{
         <div class="flex w-full justify-between">
            {
                areas_clone().iter().map(|area| {
                    let area_clone = area.clone();
                    let is_area_in_project = ids.contains(&area_clone.id);
                    let ml = if is_area_in_project { "" } else { "-ml-96" };
                    let delay = 0.05 * project_index as f32;
                    let dot_color = if is_project_hovered { "h-3 w-3 " } else { "opacity-0" };
                    let hover_style = if !is_project_hovered { "width: 0px; height: 0px;" } 
                    else { "width: 12px; height: 12px; background: blue" };
                    // logging::log!("Project Is Hovered: {}", dot_color());
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
                                class=format!(" {ml} relative bg-black  h-2 w-2 rounded-full transition-margin ease-out duration-[1s]")  
                                style=format!("transition-delay: {delay}s;")
                                // class:hidden=is_project_hovered
                                />
                                // <div 
                                // class=format!(" {ml} relative bg-black   rounded-full transition-size ease-out duration-[0.4s]")  
                                // style=format!("{hover_style}")

                                // class:hidden=!is_project_hovered
                                // />
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
                    let mut projects_vec = projects();
                    projects_vec.sort_by(|a, b| a.order.cmp(&b.order));

                    projects_vec.iter().enumerate().map(|(project_index, project)| {
                        let project_clone = project.clone();
                        let areas_ids = catalog_context.get_project_areas_ids(project_clone.id as i64);
                        let is_project_hovered = hovered_project_id().map_or(false, |id| id == project_clone.id.to_string());
                        project_line(areas_ids, project_index, is_project_hovered)
                    }).collect_view()
                }
            </div>
        }
    };
    view! {
        <div class="sticky top-0 pt-12 z-10  bg-white " >
        <div class="flex w-full grow justify-between pb-2  text-[15px]">
            {
                move || categories().into_iter().map(|cat| {
                    let cat_clone = cat.clone();
                    let cat_clone_2 = cat.clone();
                    let is_current = current_category.0.get() == Some(cat_clone);
                    view! {
                        <div
                            on:click=move |_| {
                                current_category.1.set(Some(cat_clone_2.clone()));
                            }
                            class="cursor-pointer transition-colors duration-200"
                            class:text-gray-300={!is_current}
                            >
                            <div class="truncate">{cat}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
        <div class="flex w-full grow justify-between pt-2 pb-2   ">
            {
                move ||{ 
                    let mut areas = areas();
                    areas.sort_by(|a, b| a.order.unwrap_or(0).cmp(&b.order.unwrap_or(0)));
                    areas.into_iter().map(|area| {
                    view! {
                        <div class="w-full text-center truncate relative group">
                            <div inner_html=area.to_format() class=""/>
                            // <div class="absolute w-[1px] h-4 mt-2 group-hover:h-[1000px] duration-[1s] ease-out group-hover:bg-black transition-all bg-gray-300 left-1/2 transform -translate-x-1/2" />
                            // <div class="text-sm text-gray-500">{area.desc.clone().unwrap_or_default()}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            }
        
        </div>
        </div>
        <div class="mt-[20px]">
            {move || areas_project_match()}
        </div>
    }
}
