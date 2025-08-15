use leptos::prelude::*;
use leptos::html::Input;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;
use std::collections::HashSet;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::catalog::catalog_context::use_catalog;
use crate::ui::*;

#[component]
fn AreaSelector(
    areas: ReadSignal<Vec<ProjectArea>>,
    selected_areas: ReadSignal<HashSet<i64>>,
    set_selected_areas: WriteSignal<HashSet<i64>>,
    is_submitting: ReadSignal<bool>,
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
    
    // Refs for form elements
    let title_input_ref = NodeRef::<Input>::new();
    
    let is_edit_mode = project.is_some();
    
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
            // <h2 class="text-2xl font-bold mb-6 text-gray-900">{form_title}</h2>
            
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
                                on_input=Box::new(move |ev| {
                                    set_title.set(event_target_value(&ev));
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
                                on_input=Box::new(move |ev| {
                                    set_desc.set(event_target_value(&ev));
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
                <div class="flex-1 space-y-2" style="min-height: 60vh;">
                    <FieldLabel
                        text="Project Content".to_string()
                        for_="content".to_string()
                    />
                    <textarea
                        class="w-full h-full p-3 border border-gray-300 rounded-md resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="Enter your project content, notes, or any additional information..."
                        name="content"
                        id="content"
                        disabled=is_submitting.get()
                    />
                </div>
                
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

