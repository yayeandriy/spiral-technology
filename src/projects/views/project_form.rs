use leptos::prelude::*;
use leptos::html::Input;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::catalog::catalog_context::use_catalog;
use crate::projects::views::project_content_editor::ProjectContentEditor;
use crate::ui::*;

#[derive(Clone, Debug, PartialEq)]
enum AutoSaveStatus {
    Idle,
    Pending,
    Saving,
    Saved,
    Error(String),
}

#[component]
fn AreaSelector(
    areas: ReadSignal<Vec<ProjectArea>>,
    selected_areas: ReadSignal<HashSet<i64>>,
    set_selected_areas: WriteSignal<HashSet<i64>>,
    is_submitting: ReadSignal<bool>,
    trigger_autosave: WriteSignal<bool>,
    enable_autosave: bool,
) -> impl IntoView {
    // Track which categories are expanded (all collapsed by default)
    let (expanded_categories, set_expanded_categories) = signal::<HashSet<String>>(HashSet::new());
    
    // Initialize all categories as collapsed when areas change (keep empty HashSet)
    Effect::new(move |_| {
        let _areas_list = areas.get();
        // Don't auto-expand categories - keep them collapsed by default
        set_expanded_categories.set(HashSet::new());
    });
    
    let toggle_category = move |category: String| {
        set_expanded_categories.update(|expanded| {
            if expanded.contains(&category) {
                expanded.remove(&category);
            } else {
                expanded.insert(category);
            }
        });
    };

    let expand_all = move || {
        let areas_list = areas.get();
        let mut categories = HashSet::new();
        for area in areas_list {
            categories.insert(area.category.clone());
        }
        set_expanded_categories.set(categories);
    };

    let collapse_all = move || {
        set_expanded_categories.set(HashSet::new());
    };

    view! {
        <div class="space-y-3">
            // Expand/Collapse All buttons
            <div class="flex justify-end space-x-2 mb-3">
                <button
                    type="button"
                    class="text-xs text-blue-600 hover:text-blue-800 focus:outline-none"
                    on:click=move |_| expand_all()
                >
                    "Expand All"
                </button>
                <span class="text-xs text-gray-300">"|"</span>
                <button
                    type="button"
                    class="text-xs text-blue-600 hover:text-blue-800 focus:outline-none"
                    on:click=move |_| collapse_all()
                >
                    "Collapse All"
                </button>
            </div>
            
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
                    let category_for_toggle = category.clone();
                    let is_expanded = Signal::derive(move || expanded_categories.get().contains(&category));
                    let areas_for_counter = areas_in_category.clone();
                    
                    view! {
                        <div class="mb-4">
                            <button
                                type="button"
                                class="w-full text-left flex items-center justify-between text-sm font-medium text-gray-700 mb-2 border-b border-gray-200 pb-1 hover:text-gray-900 focus:outline-none focus:text-gray-900"
                                on:click=move |_| toggle_category(category_for_toggle.clone())
                            >
                                <div class="flex items-center space-x-2">
                                    <span>{category_name.clone()}</span>
                                    {move || {
                                        // Count selected areas in this category
                                        let selected_count = areas_for_counter.iter()
                                            .filter(|area| selected_areas.get().contains(&area.id))
                                            .count();
                                        let total_count = areas_for_counter.len();
                                        
                                        if selected_count > 0 {
                                            view! {
                                                <span class="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded-full">
                                                    {format!("{}/{}", selected_count, total_count)}
                                                </span>
                                            }
                                        } else {
                                            view! {
                                                <span class="text-xs text-gray-400">
                                                    {format!("0/{}", total_count)}
                                                </span>
                                            }
                                        }
                                    }}
                                </div>
                                <span class="text-xs text-gray-500">
                                    {move || if is_expanded.get() { "▼" } else { "▶" }}
                                </span>
                            </button>
                            {move || {
                                if is_expanded.get() {
                                    view! {
                                        <div class="space-y-2 ml-2">
                                            {areas_in_category.clone().into_iter().map(|area| {
                                                let area_id = area.id;
                                                let area_title = area.title.clone();
                                                let area_desc = area.desc.clone();
                                                let selected = Signal::derive(move || selected_areas.get().contains(&area_id));
                                                
                                                view! {
                                                    <AreaCheckbox
                                                        area_id=area_id
                                                        title=area_title
                                                        description=area_desc
                                                        selected=selected
                                                        disabled=is_submitting.get()
                                                        on_change=Box::new(move |_| {
                                                            set_selected_areas.update(|areas| {
                                                                if areas.contains(&area_id) {
                                                                    areas.remove(&area_id);
                                                                } else {
                                                                    areas.insert(area_id);
                                                                }
                                                            });
                                                            // Trigger autosave signal if enabled
                                                            if enable_autosave {
                                                                trigger_autosave.set(true);
                                                            }
                                                        })
                                                    />
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
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
    
    // Clone project for use in closures
    let project_for_autosave = project.clone();
    let is_edit_mode = project.is_some();
    
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
    
    // Autosave state
    let (autosave_status, set_autosave_status) = signal(AutoSaveStatus::Idle);
    let (autosave_timeout_id, set_autosave_timeout_id) = signal::<Option<i32>>(None);
    let (area_autosave_trigger, set_area_autosave_trigger) = signal(false);
    let autosave_delay_ms = 1500; // 1.5 seconds throttling
    
    // Load existing project areas if editing
    let project_id_for_areas = project.as_ref().map(|p| p.id);
    if let Some(proj_id) = project_id_for_areas {
        let catalog_context_for_load = catalog_context.clone();
        let set_selected_areas_for_load = set_selected_areas.clone();
        spawn_local(async move {
            // First fetch catalog data
            catalog_context_for_load.fetch_catalog().await;
            // Then get areas for this project
            let project_areas = catalog_context_for_load.get_project_areas(proj_id as i64);
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
    
    // Autosave function (for project data and areas, not content)
    let autosave_project_data = {
        let project_context = project_context.clone();
        let catalog_context = catalog_context.clone();
        let set_autosave_status = set_autosave_status.clone();
        let project_for_autosave = project_for_autosave.clone();
        
        move |title_val: Option<String>, desc_val: Option<String>, area_ids: Option<HashSet<i64>>| {
            let project_context = project_context.clone();
            let catalog_context = catalog_context.clone();
            let set_autosave_status = set_autosave_status.clone();
            let project_for_autosave = project_for_autosave.clone();
            
            spawn_local(async move {
                // Only autosave if we're editing an existing project
                if let Some(proj) = project_for_autosave {
                    set_autosave_status.set(AutoSaveStatus::Saving);
                    
                    // Update project data if provided
                    if let (Some(title_val), Some(desc_val)) = (title_val, desc_val) {
                        let updated_project = Project {
                            id: proj.id,
                            title: title_val.trim().to_string(),
                            desc: if desc_val.trim().is_empty() { 
                                None 
                            } else { 
                                Some(desc_val.trim().to_string()) 
                            },
                            created_at: proj.created_at,
                        };
                        
                        project_context.update_project(updated_project).await;
                    }
                    
                    // Update areas if provided
                    if let Some(area_ids) = area_ids {
                        if let Err(e) = catalog_context.sync_project_areas(proj.id as i64, area_ids).await {
                            leptos::logging::log!("Error syncing areas during autosave: {}", e);
                            set_autosave_status.set(AutoSaveStatus::Error(e));
                            return;
                        }
                    }
                    
                    set_autosave_status.set(AutoSaveStatus::Saved);
                    
                    // Reset to idle after 3 seconds
                    {
                        let set_autosave_status = set_autosave_status.clone();
                        let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                            set_autosave_status.set(AutoSaveStatus::Idle);
                        }) as Box<dyn FnMut()>);
                        
                        web_sys::window()
                            .unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                callback.as_ref().unchecked_ref(),
                                3000,
                            )
                            .unwrap();
                            
                        callback.forget();
                    }
                } else {
                    // For new projects, just mark as pending (no save yet)
                    set_autosave_status.set(AutoSaveStatus::Pending);
                }
            });
        }
    };
    
    // Throttled autosave trigger for project data
    let trigger_autosave_data = {
        let set_autosave_timeout_id = set_autosave_timeout_id.clone();
        let autosave_project_data = autosave_project_data.clone();
        let title = title.clone();
        let desc = desc.clone();
        
        move || {
            let set_autosave_timeout_id = set_autosave_timeout_id.clone();
            let autosave_project_data = autosave_project_data.clone();
            let title = title.clone();
            let desc = desc.clone();
            
            // Clear existing timeout if any
            if let Some(timeout_id) = autosave_timeout_id.get() {
                web_sys::window()
                    .unwrap()
                    .clear_timeout_with_handle(timeout_id);
            }
            
            // Set new timeout
            let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                let title_val = title.get();
                let desc_val = desc.get();
                
                // Basic validation before autosaving
                if !title_val.trim().is_empty() && title_val.len() <= 100 && desc_val.len() <= 500 {
                    autosave_project_data(Some(title_val), Some(desc_val), None);
                }
            }) as Box<dyn FnMut()>);
            
            let timeout_id = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    autosave_delay_ms,
                )
                .unwrap();
                
            callback.forget(); // Prevent cleanup
            set_autosave_timeout_id.set(Some(timeout_id));
        }
    };
    
    // Effect to watch for area changes and trigger autosave
    Effect::new({
        let autosave_project_data = autosave_project_data.clone();
        let selected_areas = selected_areas.clone();
        let set_autosave_status = set_autosave_status.clone();
        
        move |_| {
            if area_autosave_trigger.get() && is_edit_mode {
                set_autosave_status.set(AutoSaveStatus::Pending);
                let area_ids = selected_areas.get();
                autosave_project_data(None, None, Some(area_ids));
                // Reset the trigger
                set_area_autosave_trigger.set(false);
            }
        }
    });
    
    // Refs for form elements
    let title_input_ref = NodeRef::<Input>::new();
    
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
        <div class="max-w-6xl mx-auto p-6 bg-white text-black rounded-lg shadow-md h-screen flex flex-col">
            // Autosave status indicator
            {move || {
                if is_edit_mode {
                    let status = autosave_status.get();
                    match status {
                        AutoSaveStatus::Idle => view! { <div></div> }.into_any(),
                        AutoSaveStatus::Pending => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-yellow-600 flex items-center">
                                    <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-yellow-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    "Changes pending..."
                                </span>
                            </div>
                        }.into_any(),
                        AutoSaveStatus::Saving => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-blue-600 flex items-center">
                                    <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-blue-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    "Saving..."
                                </span>
                            </div>
                        }.into_any(),
                        AutoSaveStatus::Saved => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-green-600 flex items-center">
                                    <svg class="mr-2 h-4 w-4 text-green-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                    "Changes saved"
                                </span>
                            </div>
                        }.into_any(),
                        AutoSaveStatus::Error(err) => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-red-600 flex items-center">
                                    <svg class="mr-2 h-4 w-4 text-red-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
                                    </svg>
                                    {format!("Save failed: {}", err)}
                                </span>
                            </div>
                        }.into_any(),
                    }
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
            
            <form on:submit=on_submit class="flex-1 flex flex-col space-y-6">
            // Action buttons
                <div class="flex justify-end space-x-4 pt-4">
                    {move || {
                        if let Some(_) = on_cancel {
                            view! {
                                <CancelButton
                                    on_click=Box::new(move |_| handle_cancel(()))
                                    disabled=is_submitting.get()
                                >
                                    "Cancel"
                                </CancelButton>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                    
                    <PrimaryButton
                        type_="submit".to_string()
                        disabled=is_submitting.get() || !validation_errors.get().is_empty()
                    >
                        {move || if is_submitting.get() {
                            if is_edit_mode { "Updating..." } else { "Creating..." }
                        } else {
                            if is_edit_mode { "Update Project" } else { "Create Project" }
                        }}
                    </PrimaryButton>
                </div>
                // Top section with title/desc on left, areas on right
                <div class="flex gap-8 flex-1">
                    // Left column - Title and Description
                    <div class="w-1/2 space-y-6">
                        // Title field
                        <div class="space-y-2">
                            <FieldLabel
                                text="Project Title".to_string()
                                for_="title".to_string()
                                required=true
                            />
                            <TextInput
                                value=title
                                placeholder="Enter project title...".to_string()
                                name="title".to_string()
                                id="title".to_string()
                                required=true
                                disabled=is_submitting.get()
                                state=if title_error().is_some() { InputState::Error } else { InputState::Normal }
                                on_input=Box::new({
                                    let trigger_autosave_data = trigger_autosave_data.clone();
                                    move |ev| {
                                        set_title.set(event_target_value(&ev));
                                        // Only trigger autosave for existing projects
                                        if is_edit_mode {
                                            set_autosave_status.set(AutoSaveStatus::Pending);
                                            trigger_autosave_data();
                                        }
                                    }
                                })
                                node_ref=title_input_ref
                            />
                            {title_error().map(|error| view! {
                                <FieldError error=error.to_string() />
                            })}
                        </div>
                        
                        // Description field
                        <div class="space-y-2">
                            <FieldLabel
                                text="Description".to_string()
                                for_="desc".to_string()
                            />
                            <TextAreaWithCounter
                                value=desc
                                max_length=500
                                placeholder="Enter project description (optional)...".to_string()
                                name="desc".to_string()
                                id="desc".to_string()
                                rows=4
                                disabled=is_submitting.get()
                                state=if desc_error().is_some() { InputState::Error } else { InputState::Normal }
                                on_input=Box::new({
                                    let trigger_autosave_data = trigger_autosave_data.clone();
                                    move |ev| {
                                        set_desc.set(event_target_value(&ev));
                                        // Only trigger autosave for existing projects
                                        if is_edit_mode {
                                            set_autosave_status.set(AutoSaveStatus::Pending);
                                            trigger_autosave_data();
                                        }
                                    }
                                })
                            />
                            {desc_error().map(|error| view! {
                                <FieldError error=error.to_string() />
                            })}
                        </div>
                    </div>

                    // Right column - Areas selection
                    <div class="w-1/2 space-y-2">
                        <FieldLabel text="Project Areas".to_string() />
                        <div class="h-96 overflow-y-auto border border-gray-300 rounded-md p-3 bg-gray-50">
                            <AreaSelector 
                                areas=areas_context.areas.0
                                selected_areas=selected_areas
                                set_selected_areas=set_selected_areas
                                is_submitting=is_submitting
                                trigger_autosave=set_area_autosave_trigger
                                enable_autosave=is_edit_mode
                            />
                        </div>
                        {move || {
                            let count = selected_areas.get().len();
                            let message = format!("{} area{} selected", count, if count == 1 { "" } else { "s" });
                            view! {
                                <InfoMessage message=message />
                            }
                        }}
                    </div>
                </div>

                // Bottom section - Large text area (2/3 of remaining height)
                <ProjectContentEditor />
                
                // Validation errors
                {move || {
                    let errors = validation_errors.get();
                    if !errors.is_empty() {
                        view! {
                            <ValidationErrors errors=errors />
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
                
                
            </form>
        </div>
    }
}

