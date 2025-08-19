use leptos::{logging, prelude::*, reactive::spawn_local};

use crate::{areas::{areas_context::use_areas, model::ProjectArea, views::area_editor::AreaEditor}, catalog::catalog_context::use_catalog,  ui::{select::select::Select, button::{ButtonSize, SecondaryButton}}};

#[component]
pub fn ProjectAreasEditor(
    project_id: i32,
) -> impl IntoView {
    let expanded_cat = RwSignal::new(None::<String>);

    let catalog_context = use_catalog();
    let areas_context = use_areas();
    
    let categories = Signal::derive({
        let areas_context = areas_context.clone();
        move || areas_context.categories.0.get()
    });
    let all_areas = Signal::derive({
        let areas_context = areas_context.clone();
        move || areas_context.areas.0.get()
    });

    let areas_by_category = Signal::derive({
        let all_areas = all_areas.clone();
        let categories = categories.clone();
        move || {
            let areas = all_areas.get();
            let categories = categories.get();
            let mut map = std::collections::HashMap::new();
            for category in categories {
                let category_areas: Vec<_> = areas.iter().filter(|area| area.category == category).cloned().collect();
                map.insert(category, category_areas);
            }
            map
        }
    });
    
    let project_areas_ids = Signal::derive({
        let catalog_context = catalog_context.clone();
        move || catalog_context.get_project_areas_ids(project_id as i64)
    });
    
    let project_areas = Signal::derive({
        let all_areas = all_areas.clone();
        move || {
            let ids = project_areas_ids.get();
            let areas = all_areas.get();
            areas.iter().filter(|area| ids.contains(&area.id)).map(|area| area.title.clone()).collect::<Vec<_>>()
        }
    });

    view! {
        <div>
        <For
            each=move || categories.get()
            key=|category| category.clone()
            children=move |category| {
                view! {
                    <CategorySection 
                        category=category
                        project_id=project_id
                        expanded_cat=expanded_cat
                        areas_by_category=areas_by_category
                        project_areas=project_areas
                        catalog_context=catalog_context.clone()
                        all_areas=all_areas
                    />
                }
            }
        />
        </div>
    }
}

#[component]
fn CategorySection(
    category: String,
    project_id: i32,
    expanded_cat: RwSignal<Option<String>>,
    areas_by_category: Signal<std::collections::HashMap<String, Vec<ProjectArea>>>,
    project_areas: Signal<Vec<String>>,
    catalog_context: std::sync::Arc<crate::catalog::catalog_context::CatalogContext>,
    all_areas: Signal<Vec<ProjectArea>>,
) -> impl IntoView {
    let is_expanded = Signal::derive({
        let category = category.clone();
        move || expanded_cat.get() == Some(category.clone())
    });
    
    let category_areas = Signal::derive({
        let category = category.clone();
        move || areas_by_category.get().get(&category).cloned().unwrap_or_default()
    });

    // Create a separate area_to_edit signal for this category
    let local_area_to_edit = RwSignal::new(None::<ProjectArea>);

    view! {
        <div class="border-b border-x w-full first:border-t first:rounded-t-[6px] p-2  hover:bg-gray-100 last:rounded-b-[6px]">
            <div class="text-sm mb-1 cursor-pointer transition-all flex justify-between"
            on:click={
                let category = category.clone();
                move |_| {
                    let current_expanded = expanded_cat.get();
                    if current_expanded == Some(category.clone()) {
                        expanded_cat.set(None);
                        // Clear the local area when collapsing
                        local_area_to_edit.set(None);
                    } else {
                        expanded_cat.set(Some(category.clone()));
                        // Clear any existing area when expanding
                        local_area_to_edit.set(None);
                    }
                }
            }
            >
                <div>
                    {category.clone()}
                </div>
                <div class="flex gap-1 justify-end" class:hidden=move || is_expanded.get() >
                    {
                        move || {
                            let areas = category_areas.get();
                            let project_areas_current = project_areas.get();
                            areas.iter().filter(|area| project_areas_current.iter().any(|pa| &area.title == pa))
                             .map(|area| view!{ <div class="uppercase tracking-wider text-[10px] max-w-24 truncate px-2 text-white bg-black rounded-[6px]" >{area.title.clone()}</div> }).collect_view()
                        }
                    }
                </div>
            </div>
            <Show
                when=move || is_expanded.get()
                fallback=|| view! { <div/> }
            >
                <div class="flex gap-1">
                    {
                        Select(
                            move || category_areas.get().iter().map(|area| area.title.clone()).collect::<Vec<String>>(),
                            project_areas,
                            {
                                let catalog_context = catalog_context.clone();
                                let all_areas = all_areas.clone();
                                move |selected: String| {
                                    logging::log!("Selected area title: {}", selected);
                                    let current_areas = project_areas.get();
                                    if current_areas.contains(&selected) {
                                        if let Some(area) = all_areas.get().iter().find(|area| area.title == selected) {
                                            let catalog_context_for_remove = catalog_context.clone();
                                            let area_id = area.id;
                                            spawn_local(async move {
                                                if let Err(e) = catalog_context_for_remove.remove_project_relations(project_id as i64, area_id).await {
                                                    logging::log!("Error removing area relation: {}", e);
                                                }
                                            });
                                        }
                                    } else {
                                       if let Some(area) = all_areas.get().iter().find(|area| area.title == selected) {
                                           let catalog_context_for_add = catalog_context.clone();
                                           let area_id = area.id;
                                           spawn_local(async move {
                                               if let Err(e) = catalog_context_for_add.add_project_area_relation(project_id as i64, area_id).await {
                                                   logging::log!("Error adding area relation: {}", e);
                                               }
                                           });
                                       }
                                    }
                                }
                            },
                        )
                    }
                    {
                        view!{
                            <div class="flex-col pt-[11px]" >
                            <For
                                each=move || category_areas.get()
                                key=|area| area.id
                                children=move |area| {
                                    let area_for_edit = area.clone();
                                    view!{
                                        <div class="flex mb-[7px] items-center justify-between">                
                                            <SecondaryButton 
                                            size=ButtonSize::Small
                                            on_click=move |_| {
                                                local_area_to_edit.set(Some(area_for_edit.clone()));
                                            }>
                                                {"✏️"}
                                            </SecondaryButton>
                                        </div>
                                    }
                                }
                            />      
                            </div>
                        }
                    }
                </div>
                {
                    let category_for_editor = category.clone();
                    move || {
                        let area_to_pass = local_area_to_edit.get();
                        match area_to_pass {
                            Some(area) => view! {
                                <div class="mt-2">
                                    <AreaEditor area=area category=category_for_editor.clone() />
                                </div>
                            },
                            None => view! {
                                <div class="mt-2">
                                    <AreaEditor category=category_for_editor.clone() />
                                </div>
                            }
                        }
                    }
                }
            </Show>
        </div>
    }
}
