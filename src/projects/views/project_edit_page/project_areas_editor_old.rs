use leptos::{logging, prelude::*, reactive::spawn_local};

use crate::{areas::{areas_context::use_areas, model::ProjectArea}, catalog::catalog_context::use_catalog, projects::views::project_edit_page::area_editor::AreaEditor, ui::{s_selector::s_selector::SSelector, signal_button::{ButtonSize, SSecondaryButton}}};


#[component]
pub fn ProjectAreasEditor(
    project_id: i32,
) -> impl IntoView {
    let expanded_cat = signal(None::<String>);



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
    let areas_by_category = Signal::derive(move || {
        let areas = all_areas();
        let categories = categories();
        let mut map = std::collections::HashMap::new();
        for category in categories {
            let category_areas: Vec<_> = areas.iter().filter(|area| area.category == category).cloned().collect();
            map.insert(category, category_areas);
        }
        map
    });
    
    let project_areas_ids = Signal::derive(move || catalog_context.get_project_areas_ids(project_id as i64));
    
    let project_areas = Signal::derive(move || {
        let ids = project_areas_ids.get();
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

    let area_to_edit = signal(None::<ProjectArea>);



#[component]
fn CategoryItem(
    category: String,
    project_id: i32,
    expanded_cat: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    areas_by_category: Signal<std::collections::HashMap<String, Vec<ProjectArea>>>,
    project_areas: Signal<Vec<String>>,
    area_to_edit: (ReadSignal<Option<ProjectArea>>, WriteSignal<Option<ProjectArea>>),
    catalog_context_clone_2: crate::catalog::catalog_context::CatalogContext,
    area_id_by_title: impl Fn(&str) -> Option<i32> + 'static + Clone,
) -> impl IntoView {
    let category_name = category.clone();
    let is_cat_expanded = move || expanded_cat.0.get() == Some(category_name.clone());
    
    view! {
        <div class="border-b border-x w-full first:border-t first:rounded-t-[6px] p-2  hover:bg-gray-100 last:rounded-b-[6px]">
            <div class="text-sm mb-1 cursor-pointer transition-all flex justify-between"
            on:click=move |_| {
                if expanded_cat.0.get() == Some(category_name.clone()) {
                    expanded_cat.1.set(None);
                } else {
                    expanded_cat.1.set(Some(category_name.clone()));
                }
            }
            >
                <div>
                    {category_name.clone()}
                </div>
                <div class="flex gap-1 justify-end" class:hidden=is_cat_expanded() >
                    {
                        let areas = areas_by_category.get().get(&category_name).cloned().unwrap_or_default();
                        project_areas.get().iter().filter(|area| areas.iter().any(|a| &a.title == *area))
                         .map(|area| view!{ <div class="uppercase tracking-wider text-[10px] max-w-24 truncate px-2 text-white bg-black rounded-[6px]" >{area.clone()}</div> }).collect_view()
                    }
                </div>
            </div>
            {
                move || {
                    if expanded_cat.0.get() == Some(category_name.clone()) {
                        let areas = areas_by_category.get().get(&category_name).cloned().unwrap_or_default();
                        view! {
                            <div class="flex gap-1">
                                {
                                    SSelector(
                                        move || areas.iter().map(|area| area.title.clone()).collect::<Vec<String>>(),
                                        project_areas,
                                        {
                                            let area_id_by_title_clone = area_id_by_title.clone();
                                            let catalog_context_clone_2_clone = catalog_context_clone_2.clone();
                                            move |selected: String| {
                                                logging::log!("Selected area title: {}", selected);
                                                let mut current_areas = project_areas.get();
                                                if current_areas.contains(&selected) {
                                                    if let Some(area_id) = area_id_by_title_clone(&selected) {
                                                        let catalog_context_for_remove = catalog_context_clone_2_clone.clone();
                                                        spawn_local(async move {
                                                            if let Err(e) = catalog_context_for_remove.remove_project_relations(project_id as i64, area_id).await {
                                                                logging::log!("Error removing area relation: {}", e);
                                                            }
                                                        });
                                                    }
                                                } else {
                                                   if let Some(area_id) = area_id_by_title_clone(&selected) {
                                                       let catalog_context_for_add = catalog_context_clone_2_clone.clone();
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
                                        <div class="flex-col" >
                                        { areas.iter().map(|area| {
                                                let area_for_edit = area.clone();
                                                view!{
                                                    <div class="flex items-center justify-between">                
                                                        <SSecondaryButton 
                                                        size=ButtonSize::Small
                                                        on_click=move |_| {
                                                            area_to_edit.1.set(Some(area_for_edit.clone()));
                                                        }>
                                                            {"✏️"}
                                                        </SSecondaryButton>
                                                    </div>
                                                }
                                            }).collect_view()}            
                                        </div>
                                    }
                                }
                            </div>
                            {
                                move || if let Some(area) = area_to_edit.0.get() {
                                    view! {
                                        <div class="mt-2">
                                            <AreaEditor area=area category=category_name.clone() />
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <AreaEditor category={category_name.clone()} /> }.into_any()
                                }
                            }
                        }.into_any()
                    } else {
                        view! {
                            <div/>
                        }.into_any()
                    }
                }
            }
        </div>
    }
}

#[component]
pub fn ProjectAreasEditor(
    project_id: i32,
) -> impl IntoView {
    let expanded_cat = signal(None::<String>);

    let catalog_context = use_catalog();
    let catalog_context_clone_2 = catalog_context.clone();
    let areas_context = use_areas();
    let area_context_clone = areas_context.clone();
    let area_context_clone_2 = areas_context.clone();

    let all_areas = move || area_context_clone.areas.0.get();
    let all_areas_clone = all_areas.clone();
    let all_areas_clone_2 = all_areas.clone();
    let categories = move || area_context_clone_2.categories.0.get();

    let areas_by_category = Signal::derive(move || {
        let areas = all_areas();
        let categories = categories();
        let mut map = std::collections::HashMap::new();
        for category in categories {
            let category_areas: Vec<_> = areas.iter().filter(|area| area.category == category).cloned().collect();
            map.insert(category, category_areas);
        }
        map
    });
    
    let project_areas_ids = Signal::derive(move || catalog_context.get_project_areas_ids(project_id as i64));
    
    let project_areas = Signal::derive(move || {
        let ids = project_areas_ids.get();
        all_areas_clone().iter().filter(|area| ids.contains(&area.id)).map(|area| area.title.clone()).collect::<Vec<_>>()
    });
    
    let area_id_by_title = move |title: &str| {
        all_areas_clone_2().iter().find(|area| area.title == title).map(|area| area.id)
    };

    let area_to_edit = signal(None::<ProjectArea>);

    view! {
        <div>
        {
            categories().iter().map(|category| {
                view! {
                    <CategoryItem 
                        category=category.clone()
                        project_id=project_id
                        expanded_cat=expanded_cat
                        areas_by_category=areas_by_category
                        project_areas=project_areas
                        area_to_edit=area_to_edit
                        catalog_context_clone_2=catalog_context_clone_2.clone()
                        area_id_by_title=area_id_by_title.clone()
                    />
                }
            }).collect_view()
        }
        </div>
    }
}

