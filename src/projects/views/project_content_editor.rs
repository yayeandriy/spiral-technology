use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{
    content::content_context::use_project_content, 
    ui::{button::{PrimaryButton, CancelButton, ButtonSize}, textarea::TextArea, error::ErrorMessage}
};

#[component]
pub fn ProjectContentEditor() -> impl IntoView {
    let content_context = use_project_content();
    
    // Local state for the textarea content
    let (content_text, set_content_text) = signal(String::new());
    let (has_unsaved_changes, set_has_unsaved_changes) = signal(false);
    
    // Clone context for use in different closures
    let content_context_for_effect = content_context.clone();
    let content_context_for_status = content_context.clone();
    let content_context_for_error = content_context.clone();
    let content_context_for_mode = content_context.clone();
    let content_context_for_buttons = content_context.clone();
    let content_context_for_info = content_context.clone();
    let content_context_for_debug = content_context.clone();
    let content_context_for_cancel_btn = content_context.clone();
    let content_context_for_save_btn = content_context.clone();
    
    // Initialize textarea with existing content when it loads
    Effect::new(move || {
        // Debug: Log project ID status
        leptos::logging::log!("ProjectContentEditor - Project ID: {:?}", 
            content_context_for_effect.project_id.0.get());
            
        // React to content changes - both when content loads AND when it's cleared
        let current_content = content_context_for_effect.project_content.0.get();
        
        if let Some(content) = current_content {
            let text = content.text.unwrap_or_default();
            set_content_text.set(text.clone());
            set_has_unsaved_changes.set(false);
            leptos::logging::log!("ProjectContentEditor - Loaded existing content, length: {}", text.len());
        } else {
            // Clear the textarea when there's no content (new project or project with no content)
            set_content_text.set(String::new());
            set_has_unsaved_changes.set(false);
            leptos::logging::log!("ProjectContentEditor - No content found, clearing textarea");
        }
    });

    // Debug: Add effect to log button state changes
    Effect::new(move || {
        let has_changes = has_unsaved_changes.get();
        let is_loading = content_context_for_debug.is_loading.0.get();
        let is_disabled = !has_changes || is_loading;
        leptos::logging::log!("=== BUTTON STATE DEBUG ===");
        leptos::logging::log!("Has unsaved changes: {}", has_changes);
        leptos::logging::log!("Is loading: {}", is_loading);
        leptos::logging::log!("Button should be disabled: {}", is_disabled);
        leptos::logging::log!("========================");
    });

    view! {
        <div class="flex flex-col gap-4 p-6 bg-white rounded-lg shadow-sm border">
            <div class="flex items-center justify-between">
                <h3 class="text-lg font-semibold text-gray-900">
                    "Project Content"
                </h3>
                <div class="flex items-center gap-2">
                    {
                        let has_unsaved_changes = has_unsaved_changes.clone();
                        move || {
                            if content_context_for_status.is_loading.0.get() {
                                view! {
                                    <div class="flex items-center gap-2 text-sm text-gray-500">
                                        <div class="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
                                        "Saving..."
                                    </div>
                                }.into_any()
                            } else if has_unsaved_changes.get() {
                                view! {
                                    <div class="flex items-center gap-1 text-sm text-orange-600">
                                        <div class="w-2 h-2 bg-orange-600 rounded-full"></div>
                                        "Unsaved changes"
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="flex items-center gap-1 text-sm text-green-600">
                                        <div class="w-2 h-2 bg-green-600 rounded-full"></div>
                                        "Saved"
                                    </div>
                                }.into_any()
                            }
                        }
                    }
                </div>
            </div>
            
            // Error display
            {
                move || {
                    if let Some(error) = content_context_for_error.error.0.get() {
                        view! {
                            <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-md">
                                <ErrorMessage 
                                    message=error 
                                    class="".to_string()
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            }
            
            // Content editor
            <div class="space-y-4">
                <TextArea
                    value=Signal::derive(move || content_text.get())
                    placeholder="Enter your project content here... This will create new content or update existing content.".to_string()
                    rows=10
                    on_input=Box::new({
                        let set_content_text = set_content_text.clone();
                        let set_has_unsaved_changes = set_has_unsaved_changes.clone();
                        move |ev: leptos::ev::Event| {
                            let value = event_target_value(&ev);
                            leptos::logging::log!("TextArea input detected. Value length: {}, Setting unsaved changes to true", value.len());
                            set_content_text.set(value);
                            set_has_unsaved_changes.set(true);
                        }
                    })
                    class="min-h-[200px] font-mono text-sm".to_string()
                />
                
                // Action buttons
                <div class="flex justify-between items-center">
                    <div class="text-sm text-gray-500">
                        {
                            move || {
                                if content_context_for_mode.project_content.0.get().is_some() {
                                    "Updating existing content"
                                } else {
                                    "Creating new content"
                                }
                            }
                        }
                    </div>
                    
                    <div class="flex gap-2">
                        {
                            let has_unsaved_changes = has_unsaved_changes.clone();
                            let content_context_for_cancel_btn = content_context_for_cancel_btn.clone();
                            let content_context_for_buttons = content_context_for_buttons.clone();
                            let set_content_text = set_content_text.clone();
                            let set_has_unsaved_changes = set_has_unsaved_changes.clone();
                            
                            move || {
                                let cancel_disabled = !has_unsaved_changes.get() || content_context_for_cancel_btn.is_loading.0.get();
                                view! {
                                    <CancelButton
                                        size=ButtonSize::Medium
                                        disabled=cancel_disabled
                                        on_click=Box::new({
                                            let content_context = content_context_for_buttons.clone();
                                            let set_content_text = set_content_text.clone();
                                            let set_has_unsaved_changes = set_has_unsaved_changes.clone();
                                            move |_| {
                                                if let Some(content) = content_context.project_content.0.get() {
                                                    let text = content.text.unwrap_or_default();
                                                    set_content_text.set(text);
                                                } else {
                                                    set_content_text.set(String::new());
                                                }
                                                set_has_unsaved_changes.set(false);
                                            }
                                        })
                                    >
                                        "Cancel"
                                    </CancelButton>
                                }
                            }
                        }
                        
                        {
                            let has_unsaved_changes = has_unsaved_changes.clone();
                            let content_context_for_save_btn = content_context_for_save_btn.clone();
                            let content_text = content_text.clone();
                            let set_has_unsaved_changes = set_has_unsaved_changes.clone();
                            
                            move || {
                                let save_disabled = !has_unsaved_changes.get() || content_context_for_save_btn.is_loading.0.get();
                                leptos::logging::log!("PrimaryButton disabled value: {}", save_disabled);
                                
                                let button_text = if content_context_for_save_btn.project_content.0.get().is_some() {
                                    "Update Content"
                                } else {
                                    "Create Content"
                                };
                                
                                view! {
                                    <PrimaryButton
                                        size=ButtonSize::Medium
                                        disabled=save_disabled
                                        on_click=Box::new({
                                            let content_context = content_context_for_save_btn.clone();
                                            let content_text = content_text.clone();
                                            let set_has_unsaved_changes = set_has_unsaved_changes.clone();
                                            let has_unsaved_changes = has_unsaved_changes.clone();
                                            move |_| {
                                                // Debug: Log button click first
                                                leptos::logging::log!("Save button clicked!");
                                                
                                                let content_context = content_context.clone();
                                                let text = content_text.get();
                                                let set_has_unsaved_changes = set_has_unsaved_changes.clone();
                                                
                                                // Debug: Log the project ID and text
                                                leptos::logging::log!("Attempting to save content. Project ID: {:?}, Text length: {}, Button disabled: {}", 
                                                    content_context.project_id.0.get(), text.len(), 
                                                    !has_unsaved_changes.get_untracked() || content_context.is_loading.0.get_untracked());
                                                
                                                if text.trim().is_empty() {
                                                    leptos::logging::log!("Text is empty, not saving");
                                                    return;
                                                }
                                                
                                                spawn_local(async move {
                                                    leptos::logging::log!("About to call create_or_update_project_content");
                                                    content_context.create_or_update_project_content(Some(text)).await;
                                                    leptos::logging::log!("create_or_update_project_content completed");
                                                    set_has_unsaved_changes.set(false);
                                                });
                                            }
                                        })
                                    >
                                        {button_text}
                                    </PrimaryButton>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
            
            // Content info
            {
                move || {
                    if let Some(content) = content_context_for_info.project_content.0.get() {
                        view! {
                            <div class="pt-4 border-t border-gray-200">
                                <div class="text-xs text-gray-500 space-y-1">
                                    <div>"Content ID: " {content.id}</div>
                                    {if let Some(created) = content.created_at.as_ref() {
                                        view! {
                                            <div>"Created: " {created.clone()}</div>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            }
        </div>
    }
}