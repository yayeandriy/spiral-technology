use std::sync::Arc;

use leptos::{logging, prelude::*, reactive::spawn_local};

use crate::{areas::{areas_context::{self, use_areas}, model::ProjectCategoryName}, catalog::{self, catalog_context::{self, use_catalog}}, projects::model::Project, ui::s_selector::s_selector::SSelector};


#[component]
pub fn ProjectAreasEditor(
    project_id: i32,
) -> impl IntoView {
    let expanded_cat = signal(None::<ProjectCategoryName>);



    let catalog_context = use_catalog();
    let catalog_context_clone_2 = catalog_context.clone();
    let areas_context = use_areas();
    let area_context_clone = areas_context.clone();
    let area_context_clone_2 = areas_context.clone();

    let all_areas = move || area_context_clone.areas.0.get();
    let all_areas_clone = all_areas.clone();
    let all_areas_clone_2 = all_areas.clone();
    let categories = move || area_context_clone_2.categories.0.get();
    let categories_clone = categories.clone();
    let areas_by_category = move || {
        let areas = all_areas();
        let categories = categories();
        let mut map = std::collections::HashMap::new();
        for category in categories {
            let category_areas: Vec<_> = areas.iter().filter(|area| area.category == category).cloned().collect();
            map.insert(category, category_areas);
        }
        map
    };
    
    let project_areas_ids = move || catalog_context.get_project_areas_ids(project_id as i64);
    
    let project_areas = Signal::derive(move || {
        let ids = project_areas_ids();
        all_areas_clone().iter().filter(|area| ids.contains(&area.id)).map(|area| area.title.clone()).collect::<Vec<_>>()
    });
    
    let area_id_by_title = move |title: &str| {
        all_areas_clone_2().iter().find(|area| area.title == title).map(|area| area.id)
    };

    let toggle_area = move |area_title: String| {
        let mut current_areas = project_areas.get();
        if current_areas.contains(&area_title) {
            if let Some(area_id) = area_id_by_title(&area_title) {
                spawn_local(async move {
                    if let Err(e) = catalog_context_clone_2.remove_project_relations(project_id as i64, area_id).await {
                        logging::log!("Error removing area relation: {}", e);
                    }
                });
            }
           
        } else {
           if let Some(area_id) = area_id_by_title(&area_title) {
               spawn_local(async move {
                   if let Err(e) = catalog_context_clone_2.add_project_area_relation(project_id as i64, area_id).await {
                       logging::log!("Error adding area relation: {}", e);
                   }
               });
           }
        }
    };
    let areas_by_category_clone = areas_by_category.clone();
    let category_areas_selector = move |category: ProjectCategoryName| {
        let areas = areas_by_category().get(&category).cloned().unwrap_or_default();
        let toggle_area_clone = toggle_area.clone();
        SSelector(
            move || areas.iter().map(|area| area.title.clone()).collect::<Vec<String>>(),
            project_areas,
            move |selected: String| {
                logging::log!("Selected area title: {}", selected);
                toggle_area_clone.clone()(selected);
            },
        )
    };

   let selected_areas_by_cat = move |cat: ProjectCategoryName| {
       let areas = areas_by_category_clone().get(&cat).cloned().unwrap_or_default();
       project_areas.get().iter().filter(|area| areas.iter().any(|a| &a.title == *area))
        .map(|area| view!{ <div class="uppercase tracking-wider text-[10px] max-w-24 truncate px-2 text-white bg-black rounded-[6px]" >{area.clone()}</div> }).collect_view()
   };



    view! {
        <div>
        {
            move || categories_clone().iter().map(|category| {
                let category_clone = category.clone();
                let category_clone_2 = category.clone();
                let category_clone_3 = category.clone();
                let category_areas_selector = category_areas_selector.clone();
                let expanded_cat = expanded_cat.clone();
                let expanded_cat_clone = expanded_cat.clone();
                let is_cat_expanded = move || expanded_cat.0.get() == Some(category_clone_3.clone());
                view! {
                    <div class="border-b border-x w-full first:border-t first:rounded-t-[6px] p-2  hover:bg-gray-100 last:rounded-b-[6px]">
                        <div class="text-sm mb-1 cursor-pointer transition-all flex justify-between"
                        on:click=move |_| {
                            if expanded_cat_clone.0.get() == Some(category_clone_2.clone()) {
                                expanded_cat_clone.1.set(None);
                            } else {
                                expanded_cat_clone.1.set(Some(category_clone_2.clone()));
                            }
                        }
                        >
                            <div>
                                {category_clone.clone()}
                            </div>
                            <div class="flex gap-1 justify-end" class:hidden=is_cat_expanded() >
                                {selected_areas_by_cat(category_clone_2.clone()).into_any()}
                            </div>
                        </div>
                        {
                            {
                                let category_for_check = category_clone.clone();
                                move || {
                                    if expanded_cat.0.get() == Some(category_for_check.clone()) {
                                        view! {
                                            {category_areas_selector(category_for_check.clone()).into_any()}
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div/>
                                        }.into_any()
                                    }
                                }
                            }
                        }
                        
                    </div>
                }
            }).collect_view()
        }
        </div>
                           
    }
}

