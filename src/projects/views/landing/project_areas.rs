use leptos::{html::Div, logging, prelude::*};
use leptos_use::{use_element_bounding, use_element_visibility, UseElementBoundingReturn};

use crate::{areas::areas_context::use_areas, catalog::catalog_context::use_catalog, projects::model::Project};




#[component]
pub fn ProjectAreas(
    project: ReadSignal<Option<Project>>
) -> impl IntoView {
   let catalog_context = use_catalog(); 
   let project_id = move || project.get().map(|p| p.id).unwrap_or(0);
   let areas_ids = move || catalog_context.get_project_areas_ids(project_id() as i64);
   let areas_context = use_areas();
   let areas_context_clone = areas_context.clone();
   let areas = move || areas_context_clone.get_areas_by_ids(&areas_ids());
   let areas_context_clone = areas_context.clone();
   let categories = move || areas_context_clone.categories.0.get();
   let el = NodeRef::<Div>::new();

  
    let is_visible = use_element_visibility(el);

   
    let base_class = "w-full sticky top-0 bg-white flex items-start  pr-20 pl-4 pt-4 transition-all";
    let div_class = move || {
        let visible = is_visible.get();
        if visible {
            format!("{} min-h-[200px] justify-between ", base_class)
        } else {
            format!("{} text-[13px] min-h-[100px] justify-start overflow-x-auto", base_class)
        }
        
    };

    let base_class_col = "flex  gap-1 ";
    let col_class = move || {
        if is_visible.get() {
            format!("{} flex-col gap-1", base_class_col)
        } else {
            format!("{} gap-[5px] ", base_class_col)
        }
    };

    view! {
        <div  class=div_class>
            {
                move || categories().into_iter().map(|category| {
                    let areas_in_category = areas().iter()
                        .filter(|area| &area.category == &category)
                        .cloned()
                        .collect::<Vec<_>>();
                    if !areas_in_category.is_empty() {
                        view! {
                            <div class=col_class>
                                <div class="text-gray-400">{category}</div>
                                <div class=col_class>
                                    {
                                        areas_in_category.into_iter().map(|area| {
                                            view! {
                                                <span class="bg-white pr-2">{area.title}</span>
                                            }
                                        }).collect::<Vec<_>>()
                                    }
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {<></>}.into_any()
                    }
                }).collect::<Vec<_>>()
            }
        </div>
        <div node_ref=el />
    }
}