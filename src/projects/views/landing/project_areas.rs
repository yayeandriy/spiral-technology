use leptos::prelude::*;

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

    view! {
        <div class="w-full flex justify-between items-start h-[200px] pr-20 pl-4 pt-4 transition-all">
            {
                move || categories().into_iter().map(|category| {
                    let areas_in_category = areas().iter()
                        .filter(|area| &area.category == &category)
                        .cloned()
                        .collect::<Vec<_>>();
                    if !areas_in_category.is_empty() {
                        view! {
                            <div class="flex flex-col gap-1">
                                <div class="text-gray-400">{category}</div>
                                <div class="flex flex-col gap-1">
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
    }
}