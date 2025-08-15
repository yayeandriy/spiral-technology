use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::catalog::views::area_form::AreaForm;
use crate::ui::*;

#[component]
pub fn CatalogEditor() -> impl IntoView {
    let area_context = use_areas();
    
    // State for editing
    let (editing_area, set_editing_area) = signal::<Option<ProjectArea>>(None);
    
    // Handle edit
    let handle_edit = move |area: ProjectArea| {
        set_editing_area.set(Some(area));
    };
    
    // Handle delete
    let handle_delete = {
        let area_context = area_context.clone();
        move |area_id: i64, area_title: String| {
            if web_sys::window()
                .unwrap()
                .confirm_with_message(&format!("Delete '{}'?", area_title))
                .unwrap_or(false)
            {
                let area_context_clone = area_context.clone();
                spawn_local(async move {
                    area_context_clone.delete_area(area_id).await;
                });
            }
        }
    };
    
    // Handle form success (clear editing state)
    let on_form_success = Callback::new(move |_: ()| {
        set_editing_area.set(None);
    });

    view! {
        <div class="flex flex-col">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                // Form Column
                {move || {
                    let current_editing = editing_area.get();
                    view! {
                        <AreaForm 
                            editing_area=current_editing
                            on_success=on_form_success
                        />
                    }
                }}
                
                // List Column
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold mb-4">"Areas List"</h2>
                    
                    <div class="space-y-4 max-h-96 overflow-y-auto">
                        {move || {
                            let areas_list = area_context.areas.0.get();
                            let handle_edit_fn = handle_edit.clone();
                            let handle_delete_fn = handle_delete.clone();
                            
                            // Group areas by category
                            let mut grouped_areas = std::collections::HashMap::new();
                            for area in areas_list {
                                grouped_areas
                                    .entry(area.category.clone())
                                    .or_insert_with(Vec::new)
                                    .push(area);
                            }
                            
                            // Sort categories
                            let mut categories: Vec<String> = grouped_areas.keys().cloned().collect();
                            categories.sort();
                            
                            categories.into_iter().map(|category| {
                                let areas_in_category = grouped_areas.get(&category).unwrap().clone();
                                let category_name = category.clone();
                                
                                view! {
                                    <div class="mb-6">
                                        <h3 class="text-lg font-semibold text-gray-800 mb-3 border-b border-gray-300 pb-2">
                                            {category_name}
                                        </h3>
                                        <div class="space-y-3">
                                            {areas_in_category.into_iter().map(|area| {
                                                let area_id = area.id;
                                                let area_title = area.title.clone();
                                                let area_desc = area.desc.clone();
                                                let area_for_edit = area.clone();
                                                
                                                let edit_fn = handle_edit_fn.clone();
                                                let delete_fn = handle_delete_fn.clone();
                                                
                                                view! {
                                                    <div class="p-4 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors ml-4">
                                                        <div class="flex justify-between items-start">
                                                            <div class="flex-1">
                                                                <h4 class="font-medium text-gray-900">{area_title.clone()}</h4>
                                                                {area_desc.clone().map(|desc| view! {
                                                                    <p class="text-sm text-gray-600 mt-2">{desc}</p>
                                                                })}
                                                            </div>
                                                            <div class="flex gap-2 ml-4">
                                                                <SecondaryButton
                                                                    size=ButtonSize::Small
                                                                    on_click=Box::new(move |_| edit_fn(area_for_edit.clone()))
                                                                >
                                                                    "Edit"
                                                                </SecondaryButton>
                                                                <DangerButton
                                                                    size=ButtonSize::Small
                                                                    on_click=Box::new(move |_| delete_fn(area_id, area_title.clone()))
                                                                >
                                                                    "Delete"
                                                                </DangerButton>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}