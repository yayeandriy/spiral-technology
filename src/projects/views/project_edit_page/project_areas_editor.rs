use std::sync::Arc;

use leptos::{logging, prelude::*, reactive::spawn_local};

use crate::{areas::{areas_context::{self, use_areas}, model::ProjectCategoryName}, catalog::{self, catalog_context::{self, use_catalog}}, projects::model::Project, ui::s_selector::s_selector::SSelector};


#[component]
pub fn ProjectAreasEditor(
    project_id: i32,
) -> impl IntoView {
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



    view! {
        {
            move || categories_clone().iter().map(|category| {
                let category_clone = category.clone();
                let category_areas_selector = category_areas_selector.clone();
                view! {
                    <div class="mb-4">
                        <div class="text-sm mb-1">{category_clone.clone()}</div>
                        {category_areas_selector(category_clone).into_any()}
                    </div>
                }
            }).collect::<Vec<_>>()  
        }

                           
    }
}

