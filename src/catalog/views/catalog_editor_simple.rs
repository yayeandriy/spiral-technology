use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;

use crate::areas::areas_context::use_areas;
use crate::areas::model::{ProjectArea};

#[derive(Clone, Debug, PartialEq)]
enum EditorMode {
    View,
    Create,
    Edit(ProjectArea),
}

#[component]
pub fn CatalogEditor() -> impl IntoView {
    let area_context = use_areas();
    let (current_mode, set_current_mode) = signal(EditorMode::View);
    let (selected_category_filter, set_selected_category_filter) = signal::<Option<String>>(None);
    
    // Form fields
    let (form_title, set_form_title) = signal(String::new());
    let (form_category, set_form_category) = signal(String::new());
    let (form_desc, set_form_desc) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (validation_errors, set_validation_errors) = signal::<Vec<String>>(vec![]);
    
    // Get areas and categories
    let areas = move || area_context.areas.0.get();
    let categories = move || area_context.categories.0.get();
    let is_loading = move || area_context.is_loading.0.get();
    let error = move || area_context.error.0.get();
    
    // Filter areas by category
    let filtered_areas = move || {
        let all_areas = areas();
        if let Some(filter) = selected_category_filter.get() {
            all_areas.into_iter().filter(|area| area.category == filter).collect()
        } else {
            all_areas
        }
    };
    
    // Clear form
    let clear_form = move || {
        set_form_title.set(String::new());
        set_form_category.set(String::new());
        set_form_desc.set(String::new());
        set_validation_errors.set(vec![]);
    };
    
    // Initialize form for editing
    let initialize_form_for_edit = move |area: &ProjectArea| {
        set_form_title.set(area.title.clone());
        set_form_category.set(area.category.clone());
        set_form_desc.set(area.desc.clone().unwrap_or_default());
        set_validation_errors.set(vec![]);
    };
    
    // Handle form submission
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let mut errors = vec![];
        
        if form_title.get().trim().is_empty() {
            errors.push("Area title is required".to_string());
        }
        
        if form_category.get().trim().is_empty() {
            errors.push("Category is required".to_string());
        }
        
        if form_title.get().len() > 100 {
            errors.push("Title must be less than 100 characters".to_string());
        }
        
        if form_desc.get().len() > 500 {
            errors.push("Description must be less than 500 characters".to_string());
        }
        
        set_validation_errors.set(errors.clone());
        
        if !errors.is_empty() {
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
        let current_mode_value = current_mode.get();
        
        spawn_local(async move {
            match current_mode_value {
                EditorMode::Create => {
                    area_context_clone.add_area(title_value, category_value, desc_value).await;
                }
                EditorMode::Edit(existing_area) => {
                    let updated_area = ProjectArea {
                        id: existing_area.id,
                        created_at: existing_area.created_at,
                        title: title_value,
                        category: category_value,
                        desc: desc_value,
                    };
                    area_context_clone.update_area(updated_area).await;
                }
                EditorMode::View => {}
            }
            
            set_is_submitting.set(false);
            set_current_mode.set(EditorMode::View);
            clear_form();
        });
    };
    
    // Handle delete
    let handle_delete = move |area_id: i64, area_title: String| {
        if web_sys::window()
            .unwrap()
            .confirm_with_message(&format!("Are you sure you want to delete '{}'?", area_title))
            .unwrap_or(false)
        {
            let area_context_clone = area_context.clone();
            spawn_local(async move {
                area_context_clone.delete_area(area_id).await;
            });
        }
    };
    
    // Handle edit
    let handle_edit = move |area: ProjectArea| {
        initialize_form_for_edit(&area);
        set_current_mode.set(EditorMode::Edit(area));
    };
    
    // Handle create
    let handle_create = move |_| {
        clear_form();
        set_current_mode.set(EditorMode::Create);
    };
    
    // Handle cancel
    let handle_cancel = move |_| {
        clear_form();
        set_current_mode.set(EditorMode::View);
    };
    
    view! {
        <div class="container mx-auto p-6 max-w-6xl">
            <h1 class="text-3xl font-bold mb-6 text-gray-900">"Project Areas Catalog"</h1>
            
            // Error display
            <Show 
                when=move || error().is_some()
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
                    <p class="text-red-800">"Error: " {move || error().unwrap_or_default()}</p>
                </div>
            </Show>
            
            // Loading indicator  
            <Show 
                when=is_loading
                fallback=|| view! { <div class="hidden"></div> }
            >
                <div class="mb-4 p-4 bg-blue-50 border border-blue-200 rounded-md">
                    <p class="text-blue-800">"Loading..."</p>
                </div>
            </Show>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                // Left Column: Form or Create Button
                <div class="bg-white rounded-lg shadow-md p-6">
                    <Show 
                        when=move || matches!(current_mode.get(), EditorMode::View)
                        fallback=move || view! {
                            <div>
                                <h2 class="text-xl font-semibold mb-4">
                                    {move || match current_mode.get() {
                                        EditorMode::Create => "Create New Area",
                                        EditorMode::Edit(_) => "Edit Area", 
                                        _ => ""
                                    }}
                                </h2>
                                <form on:submit=on_submit class="space-y-4">
                                    // Title field
                                    <div>
                                        <label for="title" class="block text-sm font-medium text-gray-700 mb-1">
                                            "Title" <span class="text-red-500">*</span>
                                        </label>
                                        <input
                                            type="text"
                                            id="title"
                                            name="title"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                            placeholder="Enter area title..."
                                            prop:value=form_title
                                            on:input=move |ev| set_form_title.set(event_target_value(&ev))
                                            disabled=is_submitting
                                        />
                                    </div>
                                    
                                    // Category field
                                    <div>
                                        <label for="category" class="block text-sm font-medium text-gray-700 mb-1">
                                            "Category" <span class="text-red-500">*</span>
                                        </label>
                                        <div class="flex gap-2">
                                            <input
                                                type="text"
                                                id="category"
                                                name="category"
                                                class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                                placeholder="Enter or select category..."
                                                prop:value=form_category
                                                on:input=move |ev| set_form_category.set(event_target_value(&ev))
                                                disabled=is_submitting
                                            />
                                            <select
                                                class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                on:change=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    if !value.is_empty() && value != "custom" {
                                                        set_form_category.set(value);
                                                    }
                                                }
                                                disabled=is_submitting
                                            >
                                                <option value="">"Select existing..."</option>
                                                <For
                                                    each=move || {
                                                        let mut unique_categories: Vec<String> = categories()
                                                            .into_iter()
                                                            .collect::<std::collections::HashSet<_>>()
                                                            .into_iter()
                                                            .collect();
                                                        unique_categories.sort();
                                                        unique_categories
                                                    }
                                                    key=|category| category.clone()
                                                    children=move |category| {
                                                        let category_clone = category.clone();
                                                        view! {
                                                            <option value={category}>{category_clone}</option>
                                                        }
                                                    }
                                                />
                                                <option value="custom">"+ New category"</option>
                                            </select>
                                        </div>
                                    </div>
                                    
                                    // Description field
                                    <div>
                                        <label for="desc" class="block text-sm font-medium text-gray-700 mb-1">
                                            "Description"
                                        </label>
                                        <textarea
                                            id="desc"
                                            name="desc"
                                            rows="3"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                            placeholder="Enter area description (optional)..."
                                            prop:value=form_desc
                                            on:input=move |ev| set_form_desc.set(event_target_value(&ev))
                                            disabled=is_submitting
                                        ></textarea>
                                        <p class="text-xs text-gray-500 mt-1">
                                            {move || format!("{}/500 characters", form_desc.get().len())}
                                        </p>
                                    </div>
                                    
                                    // Validation errors
                                    <Show 
                                        when=move || !validation_errors.get().is_empty()
                                        fallback=|| view! { <div class="hidden"></div> }
                                    >
                                        <div class="p-3 bg-red-50 border border-red-200 rounded-md">
                                            <ul class="text-sm text-red-700 list-disc list-inside">
                                                <For
                                                    each=move || validation_errors.get()
                                                    key=|error| error.clone()
                                                    children=move |error| view! {
                                                        <li>{error}</li>
                                                    }
                                                />
                                            </ul>
                                        </div>
                                    </Show>
                                    
                                    // Action buttons
                                    <div class="flex justify-end gap-3 pt-4">
                                        <button
                                            type="button"
                                            on:click=handle_cancel
                                            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
                                            disabled=is_submitting
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            type="submit"
                                            class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
                                            disabled=is_submitting
                                        >
                                            {move || if is_submitting.get() {
                                                "Saving..."
                                            } else {
                                                match current_mode.get() {
                                                    EditorMode::Create => "Create Area",
                                                    EditorMode::Edit(_) => "Update Area",
                                                    _ => "Submit"
                                                }
                                            }}
                                        </button>
                                    </div>
                                </form>
                            </div>
                        }
                    >
                        <div class="text-center py-8">
                            <h2 class="text-xl font-semibold mb-4">"Manage Project Areas"</h2>
                            <p class="text-gray-600 mb-6">"Create new areas or edit existing ones"</p>
                            <button
                                on:click=handle_create
                                class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                            >
                                "Create New Area"
                            </button>
                        </div>
                    </Show>
                </div>
                
                // Right Column: Areas List
                <div class="bg-white rounded-lg shadow-md p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-xl font-semibold">"Areas List"</h2>
                        // Category Filter
                        <select
                            class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                if value.is_empty() || value == "all" {
                                    set_selected_category_filter.set(None);
                                } else {
                                    set_selected_category_filter.set(Some(value));
                                }
                            }
                        >
                            <option value="all">"All Categories"</option>
                            <For
                                each=move || categories()
                                key=|category| category.clone()
                                children=move |category| {
                                    let category_clone = category.clone();
                                    view! {
                                        <option value={category}>{category_clone}</option>
                                    }
                                }
                            />
                        </select>
                    </div>
                    
                    <div class="space-y-3 max-h-96 overflow-y-auto">
                        <Show 
                            when=move || !filtered_areas().is_empty()
                            fallback=|| view! {
                                <div class="text-gray-500 text-center py-8">
                                    "No areas found."
                                </div>
                            }
                        >
                            <For
                                each=filtered_areas
                                key=|area| area.id
                                children=move |area| {
                                    let area_for_edit = area.clone();
                                    let area_for_delete = area.clone();
                                    view! {
                                        <div class="p-4 border border-gray-200 rounded-lg hover:bg-gray-50">
                                            <div class="flex justify-between items-start">
                                                <div class="flex-1">
                                                    <h3 class="font-medium">{area.title.clone()}</h3>
                                                    <p class="text-sm text-blue-600 mt-1">
                                                        <span class="bg-blue-100 px-2 py-1 rounded-full text-xs">
                                                            {area.category.clone()}
                                                        </span>
                                                    </p>
                                                    {area.desc.clone().map(|desc| view! {
                                                        <p class="text-sm text-gray-600 mt-2">{desc}</p>
                                                    })}
                                                </div>
                                                <div class="flex gap-2 ml-4">
                                                    <button
                                                        on:click=move |_| handle_edit(area_for_edit.clone())
                                                        class="px-3 py-1 text-sm bg-blue-100 text-blue-700 rounded hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                    >
                                                        "Edit"
                                                    </button>
                                                    <button
                                                        on:click=move |_| handle_delete(area_for_delete.id, area_for_delete.title.clone())
                                                        class="px-3 py-1 text-sm bg-red-100 text-red-700 rounded hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-red-500"
                                                    >
                                                        "Delete"
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}
