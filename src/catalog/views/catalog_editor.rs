use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;
use std::collections::HashSet;

use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;

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
                    <div class="relative">
                        <select
                            class="w-full text-black px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 appearance-none bg-white"
                            prop:value=move || {
                                let current = form_category.get();
                                let areas = areas.get();
                                if current.is_empty() {
                                    String::new()
                                } else if areas.iter().any(|area| area.category == current) {
                                    current
                                } else {
                                    "__custom__".to_string()
                                }
                            }
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                if value == "__custom__" {
                                    set_form_category.set(String::new());
                                } else {
                                    set_form_category.set(value);
                                }
                            }
                            disabled=move || is_submitting.get()
                        >
                            <option value="">
                                "Select a category..."
                            </option>
                            {categories.into_iter().map(|category| {
                                let cat_name = category.clone();
                                view! {
                                    <option value=category>
                                        {cat_name}
                                    </option>
                                }
                            }).collect::<Vec<_>>()}
                            <option value="__custom__">
                                "Add new category..."
                            </option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                                <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/>
                            </svg>
                        </div>
                    </div>
                    
                    // Custom category input (shown when custom category is needed)
                    <Show when=move || {
                        let current = form_category.get();
                        let areas = areas.get();
                        current == "__custom__" || (!current.is_empty() && !areas.iter().any(|area| area.category == current))
                    }>
                        <input
                            type="text"
                            class="w-full px-3 py-2 mt-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter new category name..."
                            prop:value=move || {
                                let current = form_category.get();
                                if current == "__custom__" { String::new() } else { current }
                            }
                            on:input=move |ev| set_form_category.set(event_target_value(&ev))
                            disabled=move || is_submitting.get()
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
                            <label class="block text-sm font-medium text-gray-700 mb-1">
                                "Title*"
                            </label>
                            <input
                                type="text"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="Enter area title..."
                                prop:value=form_title
                                on:input=move |ev| set_form_title.set(event_target_value(&ev))
                                disabled=is_submitting
                            />
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">
                                "Category*"
                            </label>
                            <CategorySelector 
                                areas=area_context_for_form.areas.0
                                form_category=form_category
                                set_form_category=set_form_category
                                is_submitting=is_submitting
                            />
                        </div>
                        
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">
                                "Description"
                            </label>
                            <textarea
                                rows="3"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                placeholder="Optional description..."
                                prop:value=form_desc
                                on:input=move |ev| set_form_desc.set(event_target_value(&ev))
                                disabled=is_submitting
                            ></textarea>
                        </div>
                        
                        <div class="flex justify-end gap-3">
                            <button
                                type="button"
                                on:click=move |_| clear_form()
                                class="px-4 py-2 text-gray-700 bg-gray-200 rounded hover:bg-gray-300 disabled:opacity-50"
                                disabled=is_submitting
                            >
                                "Cancel"
                            </button>
                            <button
                                type="submit"
                                class="px-4 py-2 text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50"
                                disabled=is_submitting
                            >
                                {move || if is_submitting.get() { 
                                    "Saving..." 
                                } else if editing_id.get().is_some() { 
                                    "Update Area" 
                                } else { 
                                    "Create Area" 
                                }}
                            </button>
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
                                                                <button
                                                                    on:click=move |_| edit_fn(area_for_edit.clone())
                                                                    class="px-3 py-1 text-sm bg-blue-100 text-blue-700 rounded hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                                >
                                                                    "Edit"
                                                                </button>
                                                                <button
                                                                    on:click=move |_| delete_fn(area_id, area_title.clone())
                                                                    class="px-3 py-1 text-sm bg-red-100 text-red-700 rounded hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-red-500"
                                                                >
                                                                    "Delete"
                                                                </button>
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