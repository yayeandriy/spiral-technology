use leptos::prelude::*;
use leptos::html::Input;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;
use std::collections::HashSet;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::projects::views::projects_list::ProjectsList;
use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::catalog::catalog_context::use_catalog;

#[component]
fn AreaSelector(
    areas: ReadSignal<Vec<ProjectArea>>,
    selected_areas: ReadSignal<HashSet<i64>>,
    set_selected_areas: WriteSignal<HashSet<i64>>,
    is_submitting: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="space-y-3">
            {move || {
                let areas_list = areas.get();
                
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
                        <div class="mb-4">
                            <h4 class="text-sm font-medium text-gray-700 mb-2 border-b border-gray-200 pb-1">
                                {category_name}
                            </h4>
                            <div class="space-y-2 ml-2">
                                {areas_in_category.into_iter().map(|area| {
                                    let area_id = area.id;
                                    let area_title = area.title.clone();
                                    let area_desc = area.desc.clone();
                                    
                                    view! {
                                        <div class="flex items-start space-x-3">
                                            <input
                                                type="checkbox"
                                                id=format!("area_{}", area_id)
                                                class="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                                prop:checked=move || selected_areas.get().contains(&area_id)
                                                on:change=move |_| {
                                                    set_selected_areas.update(|areas| {
                                                        if areas.contains(&area_id) {
                                                            areas.remove(&area_id);
                                                        } else {
                                                            areas.insert(area_id);
                                                        }
                                                    });
                                                }
                                                disabled=is_submitting
                                            />
                                            <label 
                                                for=format!("area_{}", area_id)
                                                class="flex-1 cursor-pointer"
                                            >
                                                <div class="text-sm font-medium text-gray-900">
                                                    {area_title}
                                                </div>
                                                {area_desc.map(|desc| view! {
                                                    <div class="text-xs text-gray-500 mt-1">
                                                        {desc}
                                                    </div>
                                                })}
                                            </label>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}

#[component]
pub fn ProjectForm(
    #[prop(optional)] project: Option<Project>,
    #[prop(optional)] on_save: Option<Callback<Project>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
) -> impl IntoView {
    let project_context = use_project();
    let areas_context = use_areas();
    let catalog_context = use_catalog();
    
    // Initialize form fields with existing project data if editing
    let (title, set_title) = signal(
        project.as_ref().map(|p| p.title.clone()).unwrap_or_default()
    );
    let (desc, set_desc) = signal(
        project.as_ref().and_then(|p| p.desc.clone()).unwrap_or_default()
    );
    let (selected_areas, set_selected_areas) = signal::<HashSet<i64>>(HashSet::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (validation_errors, set_validation_errors) = signal::<Vec<String>>(vec![]);
    
    // Load existing project areas if editing
    let project_id_for_areas = project.as_ref().map(|p| p.id);
    if let Some(proj_id) = project_id_for_areas {
        let catalog_context_for_load = catalog_context.clone();
        let set_selected_areas_for_load = set_selected_areas.clone();
        spawn_local(async move {
            // First fetch catalog data
            catalog_context_for_load.fetch_catalog().await;
            // Then get areas for this project
            let project_areas = catalog_context_for_load.get_project_areas(proj_id as i64).await;
            let area_ids: HashSet<i64> = project_areas.into_iter().collect();
            set_selected_areas_for_load.set(area_ids);
        });
    }
    
    // Load areas when component mounts
    {
        let areas_context = areas_context.clone();
        spawn_local(async move {
            areas_context.fetch_areas().await;
        });
    }
    
    // Helper function to sync project-area relations
    let sync_project_areas = {
        let catalog_context = catalog_context.clone();
        move |project_id: i32, area_ids: HashSet<i64>| async move {
            catalog_context.sync_project_areas(project_id as i64, area_ids).await
        }
    };
    
    // Refs for form elements
    let title_input_ref = NodeRef::<Input>::new();
    
    let is_edit_mode = project.is_some();
    let form_title = if is_edit_mode { "Edit Project" } else { "Create New Project" };
    
    // Validation function
    let validate_form = move || -> Vec<String> {
        let mut errors = vec![];
        
        if title.get().trim().is_empty() {
            errors.push("Project title is required".to_string());
        }
        
        if title.get().len() > 100 {
            errors.push("Project title must be less than 100 characters".to_string());
        }
        
        if desc.get().len() > 500 {
            errors.push("Description must be less than 500 characters".to_string());
        }
        
        errors
    };
    
    // Handle form submission
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let errors = validate_form();
        set_validation_errors.set(errors.clone());
        
        if !errors.is_empty() {
            return;
        }
        
        set_is_submitting.set(true);
        
        let title_value = title.get().trim().to_string();
        let desc_value = if desc.get().trim().is_empty() { 
            None 
        } else { 
            Some(desc.get().trim().to_string()) 
        };
        
        let project_context_clone = project_context.clone();
        let on_save_callback = on_save;
        let current_project = project.clone();
        let selected_areas_value = selected_areas.get();
        let sync_areas_fn = sync_project_areas.clone();
        
        spawn_local(async move {
            if is_edit_mode {
                // Update existing project
                if let Some(proj) = current_project {
                    let updated_project = Project {
                        id: proj.id,
                        title: title_value.clone(),
                        desc: desc_value.clone(),
                        created_at: proj.created_at,
                    };
                    
                    // Update project first
                    project_context_clone.update_project(updated_project.clone()).await;
                    
                    // Then sync areas
                    if let Err(e) = sync_areas_fn(proj.id, selected_areas_value.clone()).await {
                        leptos::logging::log!("Error syncing areas: {}", e);
                    }
                    
                    if let Some(callback) = on_save_callback {
                        callback.run(updated_project);
                    }
                }
            } else {
                // Create new project
                if let Some(created_project) = project_context_clone.add_project(title_value.clone(), desc_value.clone()).await {
                    // Sync areas for the new project
                    if !selected_areas_value.is_empty() {
                        if let Err(e) = sync_areas_fn(created_project.id, selected_areas_value.clone()).await {
                            leptos::logging::log!("Error syncing areas for new project: {}", e);
                        }
                    }
                    
                    if let Some(callback) = on_save_callback {
                        callback.run(created_project);
                    }
                } else {
                    leptos::logging::log!("Failed to create project");
                }
            }
            
            set_is_submitting.set(false);
        });
    };
    
    // Handle cancel
    let handle_cancel = move |_| {
        if let Some(callback) = on_cancel {
            callback.run(());
        }
    };
    
    // Real-time validation
    let title_error = move || {
        let current_title = title.get();
        if current_title.trim().is_empty() {
            Some("Title is required")
        } else if current_title.len() > 100 {
            Some("Title too long (max 100 characters)")
        } else {
            None
        }
    };
    
    let desc_error = move || {
        let current_desc = desc.get();
        if current_desc.len() > 500 {
            Some("Description too long (max 500 characters)")
        } else {
            None
        }
    };

    view! {
        <div class="max-w-2xl mx-auto p-6 bg-white text-black rounded-lg shadow-md">
            <h2 class="text-2xl font-bold mb-6 text-gray-900">{form_title}</h2>
            
            <form on:submit=on_submit class="space-y-6">
                // Title field
                <div class="space-y-2">
                    <label for="title" class="block text-sm font-medium text-gray-700">
                        "Project Title" <span class="text-red-500">*</span>
                    </label>
                    <input
                        node_ref=title_input_ref
                        type="text"
                        id="title"
                        name="title"
                        class=move || format!(
                            "w-full px-3 py-2 text-black border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 {}",
                            if title_error().is_some() { "border-red-300 focus:border-red-500" } else { "border-gray-300 focus:border-blue-500" }
                        )
                        placeholder="Enter project title..."
                        prop:value=title
                        on:input=move |ev| {
                            set_title.set(event_target_value(&ev));
                        }
                        disabled=is_submitting
                    />
                    {move || title_error().map(|error| view! {
                        <p class="text-sm text-red-600">{error}</p>
                    })}
                </div>
                
                // Description field
                <div class="space-y-2">
                    <label for="desc" class="block text-sm font-medium text-gray-700">
                        "Description"
                    </label>
                    <textarea
                        id="desc"
                        name="desc"
                        rows="4"
                        class=move || format!(
                            "w-full px-3 py-2 border text-black rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 {}",
                            if desc_error().is_some() { "border-red-300 focus:border-red-500" } else { "border-gray-300 focus:border-blue-500" }
                        )
                        placeholder="Enter project description (optional)..."
                        prop:value=desc
                        on:input=move |ev| {
                            set_desc.set(event_target_value(&ev));
                        }
                        disabled=is_submitting
                    ></textarea>
                    {move || desc_error().map(|error| view! {
                        <p class="text-sm text-red-600">{error}</p>
                    })}
                    <p class="text-xs text-gray-500">
                        {move || format!("{}/500 characters", desc.get().len())}
                    </p>
                </div>
                
                // Areas selection field
                <div class="space-y-2">
                    <label class="block text-sm font-medium text-gray-700">
                        "Project Areas"
                    </label>
                    <div class="max-h-64 overflow-y-auto border border-gray-300 rounded-md p-3 bg-gray-50">
                        <AreaSelector 
                            areas=areas_context.areas.0
                            selected_areas=selected_areas
                            set_selected_areas=set_selected_areas
                            is_submitting=is_submitting
                        />
                    </div>
                    <p class="text-xs text-gray-500">
                        {move || {
                            let count = selected_areas.get().len();
                            format!("{} area{} selected", count, if count == 1 { "" } else { "s" })
                        }}
                    </p>
                </div>
                
                // Validation errors
                {move || {
                    let errors = validation_errors.get();
                    if !errors.is_empty() {
                        view! {
                            <div class="p-4 bg-red-50 border border-red-200 rounded-md">
                                <div class="flex">
                                    <div class="ml-3">
                                        <h3 class="text-sm font-medium text-red-800">
                                            "Please fix the following errors:"
                                        </h3>
                                        <ul class="mt-2 text-sm text-red-700 list-disc list-inside">
                                            {errors.into_iter().map(|error| view! {
                                                <li>{error}</li>
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
                
                // Action buttons
                <div class="flex justify-end space-x-4 pt-4">
                    {move || {
                        if let Some(_) = on_cancel {
                            view! {
                                <button
                                    type="button"
                                    on:click=handle_cancel
                                    disabled=is_submitting
                                    class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    "Cancel"
                                </button>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                    
                    <button
                        type="submit"
                        disabled=move || is_submitting.get() || !validation_errors.get().is_empty()
                        class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {move || if is_submitting.get() {
                            if is_edit_mode { "Updating..." } else { "Creating..." }
                        } else {
                            if is_edit_mode { "Update Project" } else { "Create Project" }
                        }}
                    </button>
                </div>
            </form>
        </div>
    }
}


// Example usage component
#[component]
pub fn ProjectFormPage() -> impl IntoView {
    let (show_form, set_show_form) = signal(false);
    let (current_project, set_current_project) = signal::<Option<Project>>(None);
    let project_context = use_project();
    
    let handle_create = move |_| {
        set_current_project.set(None);
        set_show_form.set(true);
    };
    
    let handle_edit = move |project: Project| {
        set_current_project.set(Some(project));
        set_show_form.set(true);
    };
    
    let handle_delete = move |project_id: i32| {
        let context = project_context.clone();
        spawn_local(async move {
            context.delete_project(project_id).await;
        });
    };
    
    let handle_save = move |_project: Project| {
        set_show_form.set(false);
        set_current_project.set(None);
        // Optionally refetch projects or handle the saved project
    };
    
    let handle_cancel = move |_| {
        set_show_form.set(false);
        set_current_project.set(None);
    };

    view! {
        <div class="container mx-auto p-4 text-black">
            {
                let handle_save_clone = handle_save.clone();
                let handle_cancel_clone = handle_cancel.clone();
                let handle_create_clone = handle_create.clone();
                let handle_edit_clone = handle_edit.clone();
                let handle_delete_clone = handle_delete.clone();
                
                move || {
                    if show_form.get() {
                        let project_to_edit = current_project.get();
                        let save_callback = handle_save_clone.clone();
                        let cancel_callback = handle_cancel_clone.clone();
                        if let Some(project) = project_to_edit {
                            // Edit mode
                            view! {
                                <ProjectForm
                                    project=project
                                    on_save=Callback::new(save_callback)
                                    on_cancel=Callback::new(cancel_callback)
                                />
                            }.into_any()
                        } else {
                            // Create mode
                            view! {
                                <ProjectForm
                                    on_save=Callback::new(save_callback)
                                    on_cancel=Callback::new(cancel_callback)
                                />
                            }.into_any()
                        }
                    } else {
                        let create_callback = handle_create_clone.clone();
                        let edit_callback = handle_edit_clone.clone();
                        let delete_callback = handle_delete_clone.clone();
                        view! {
                            <ProjectsList 
                                on_create=Callback::new(create_callback)
                                on_edit=Callback::new(edit_callback)
                                on_delete=Callback::new(delete_callback)
                            />
                        }.into_any()
                    }
                }
            }
        </div>
    }
}
