use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{MouseEvent, SubmitEvent};
use std::collections::HashSet;

use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::ui::signal_button::{SCancelButton, SPrimaryButton};
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
pub fn AreaForm(
    editing_area: Option<ProjectArea>,
    #[prop(optional, into)] on_success: Option<Callback<()>>,
) -> impl IntoView {
    let area_context = use_areas();
    
    // Form fields - initialize with editing data if provided
    let (form_title, set_form_title) = signal(
        editing_area.as_ref().map(|a| a.title.clone()).unwrap_or_default()
    );
    let (form_category, set_form_category) = signal(
        editing_area.as_ref().map(|a| a.category.clone()).unwrap_or_default()
    );
    let (form_desc, set_form_desc) = signal(
        editing_area.as_ref().and_then(|a| a.desc.clone()).unwrap_or_default()
    );
    let (is_submitting, set_is_submitting) = signal(false);
    
    // Track editing ID
    let editing_id = editing_area.as_ref().map(|a| a.id);
    
    // Clear form
    let clear_form = move || {
        set_form_title.set(String::new());
        set_form_category.set(String::new());
        set_form_desc.set(String::new());
    };
    
    // Handle form submission
    let on_submit = {
        let area_context = area_context.clone();
        let on_success = on_success.clone();
        
        move |ev: MouseEvent| {
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
            let on_success_clone = on_success.clone();
            
            spawn_local(async move {
                if let Some(edit_id) = editing_id {
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
                
                // Call success callback if provided
                if let Some(callback) = on_success_clone {
                    callback.run(());
                }
            });
        }
    };

    let handle_submit = move |ev: MouseEvent| {
        ev.prevent_default();
        on_submit(ev);
    };

    let handle_cancel = move |_| {
        clear_form();
        if let Some(callback) = on_success {
            callback.run(());
        }
    };
    
    view! {
        <div class="bg-white rounded-lg shadow-md p-6">
            <h2 class="text-xl font-semibold mb-4">
                {if editing_id.is_some() { "Edit Area" } else { "Create New Area" }}
            </h2>
            
            <form class="space-y-4">
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
                        areas=area_context.areas.0
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
                    <SCancelButton
                        on_click=handle_cancel
                        disabled=is_submitting.get()
                    >
                        "Cancel"
                    </SCancelButton>
                    <SPrimaryButton
                        type_="submit".to_string()
                        disabled=is_submitting.get()
                        on_click=handle_submit
                    >
                        {move || if is_submitting.get() { 
                            "Saving..." 
                        } else if editing_id.is_some() { 
                            "Update Area" 
                        } else { 
                            "Create Area" 
                        }}
                    </SPrimaryButton>
                </div>
            </form>
        </div>
    }
}
