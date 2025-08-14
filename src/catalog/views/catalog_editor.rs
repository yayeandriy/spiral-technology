use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;
use std::collections::HashSet;

use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::ui::*;

#[component]
fn CategorySelector(
    areas: ReadSignal<Vec<ProjectArea>>,
    form_category: ReadSignal<String>,
    set_form_category: WriteSignal<String>,
    is_submitting: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div>
            {move || {
                let areas_list = areas.get();
                let unique_categories: HashSet<String> = areas_list
                    .iter()
                    .map(|area| area.category.clone())
                    .collect();
                let mut categories: Vec<String> = unique_categories.into_iter().collect();
                categories.sort();
                
                view! {
                    <div>
                        <CategorySelect
                            value=Signal::derive(move || form_category.get())
                            categories=categories
                            disabled=is_submitting.get()
                            on_change=Box::new(move |ev| {
                                let value = event_target_value(&ev);
                                if value == "__custom__" {
                                    set_form_category.set(String::new());
                                } else {
                                    set_form_category.set(value);
                                }
                            })
                        />
                    </div>
                    
                    // Custom category input (shown when custom category is needed)
                    <Show when=move || {
                        let current = form_category.get();
                        let areas = areas.get();
                        current == "__custom__" || (!current.is_empty() && !areas.iter().any(|area| area.category == current))
                    }>
                        <TextInput
                            value=Signal::derive(move || {
                                let current = form_category.get();
                                if current == "__custom__" { String::new() } else { current }
                            })
                            placeholder="Enter new category name...".to_string()
                            class="mt-2".to_string()
                            disabled=is_submitting.get()
                            on_input=Box::new(move |ev| set_form_category.set(event_target_value(&ev)))
                        />
                    </Show>
                }
            }}
        </div>
    }
}

#[component]
pub fn CatalogEditor() -> impl IntoView {
    let area_context = use_areas();
    let area_context_for_form = area_context.clone();
    let area_context_for_list = area_context.clone();
    
    // Form fields
    let (form_title, set_form_title) = signal(String::new());
    let (form_category, set_form_category) = signal(String::new());
    let (form_desc, set_form_desc) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (editing_id, set_editing_id) = signal::<Option<i64>>(None);
    
    // Clear form
    let clear_form = move || {
        set_form_title.set(String::new());
        set_form_category.set(String::new());
        set_form_desc.set(String::new());
        set_editing_id.set(None);
    };
    
    // Handle form submission
    let on_submit = {
        let area_context = area_context_for_form.clone();
        move |ev: SubmitEvent| {
            ev.prevent_default();
            
            if form_title.get().trim().is_empty() || form_category.get().trim().is_empty() {
                return;
            }
            
            set_is_submitting.set(true);
            
            let title_value = form_title.get().trim().to_string();
            let category_value = form_category.get().trim().to_string();
            let desc_value = if form_desc.get().trim().is_empty() {
                None
            } else {
                Some(form_desc.get().trim().to_string())
            };
            
            let area_context_clone = area_context.clone();
            let current_editing_id = editing_id.get();
            
            spawn_local(async move {
                if let Some(edit_id) = current_editing_id {
                    // Update existing area
                    let updated_area = ProjectArea {
                        id: edit_id,
                        created_at: None, // Will be preserved by the update
                        title: title_value,
                        category: category_value,
                        desc: desc_value,
                    };
                    area_context_clone.update_area(updated_area).await;
                } else {
                    // Create new area
                    area_context_clone.add_area(title_value, category_value, desc_value).await;
                }
                
                set_is_submitting.set(false);
                clear_form();
            });
        }
    };
    
    // Handle edit
    let handle_edit = {
        let set_form_title = set_form_title.clone();
        let set_form_category = set_form_category.clone();
        let set_form_desc = set_form_desc.clone();
        let set_editing_id = set_editing_id.clone();
        move |area: ProjectArea| {
            set_form_title.set(area.title);
            set_form_category.set(area.category);
            set_form_desc.set(area.desc.unwrap_or_default());
            set_editing_id.set(Some(area.id));
        }
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
    
    view! {
        <div class="container mx-auto p-6 max-w-6xl">
            <h1 class="text-3xl font-bold mb-6">"Project Areas Catalog"</h1>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                // Form Column
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold mb-4">
                        {move || if editing_id.get().is_some() { "Edit Area" } else { "Create New Area" }}
                    </h2>
                    
                    <form on:submit=on_submit class="space-y-4">
                        <div>
                            <FieldLabel
                                text="Title".to_string()
                                required=true
                            />
                            <TextInput
                                value=Signal::derive(move || form_title.get())
                                placeholder="Enter area title...".to_string()
                                disabled=is_submitting.get()
                                on_input=Box::new(move |ev| set_form_title.set(event_target_value(&ev)))
                            />
                        </div>
                        
                        <div>
                            <FieldLabel
                                text="Category".to_string()
                                required=true
                            />
                            <CategorySelector 
                                areas=area_context_for_form.areas.0
                                form_category=form_category
                                set_form_category=set_form_category
                                is_submitting=is_submitting
                            />
                        </div>
                        
                        <div>
                            <FieldLabel text="Description".to_string() />
                            <TextArea
                                value=Signal::derive(move || form_desc.get())
                                placeholder="Optional description...".to_string()
                                rows=3
                                disabled=is_submitting.get()
                                on_input=Box::new(move |ev| set_form_desc.set(event_target_value(&ev)))
                            />
                        </div>
                        
                        <div class="flex justify-end gap-3">
                            <CancelButton
                                on_click=Box::new(move |_| clear_form())
                                disabled=is_submitting.get()
                            >
                                "Cancel"
                            </CancelButton>
                            <PrimaryButton
                                type_="submit".to_string()
                                disabled=is_submitting.get()
                            >
                                {move || if is_submitting.get() { 
                                    "Saving..." 
                                } else if editing_id.get().is_some() { 
                                    "Update Area" 
                                } else { 
                                    "Create Area" 
                                }}
                            </PrimaryButton>
                        </div>
                    </form>
                </div>
                
                // List Column
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-semibold mb-4">"Areas List"</h2>
                    
                    <div class="space-y-4 max-h-96 overflow-y-auto">
                        {move || {
                            let areas_list = area_context_for_list.areas.0.get();
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