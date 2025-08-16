use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::ui::signal_button::SPrimaryButton;

use super::{
    button::{CancelButton, ButtonSize}, 
    textarea::TextArea, 
    error::ErrorMessage
};

#[derive(Clone, Debug)]
pub enum EditorMode {
    Create,
    Update,
}

#[derive(Clone, Debug)]
pub enum EditorStatus {
    Idle,
    Saving,
    Saved,
    HasUnsavedChanges,
}

#[component]
pub fn TextEditor(
    /// Text content signal for read and write
    #[prop(into)]
    content: RwSignal<String>,
    /// Current status of the editor
    #[prop(into)]
    status: Signal<EditorStatus>,
    /// Error message, if any
    #[prop(into)]
    error: Signal<Option<String>>,
    /// Editor mode (Create or Update)
    #[prop(into)]
    mode: Signal<EditorMode>,
    /// Signal to trigger save operation
    #[prop(into)]
    save_trigger: RwSignal<bool>,
    /// Signal to trigger cancel operation
    #[prop(into)]
    cancel_trigger: RwSignal<bool>,
    /// Placeholder text for the textarea
    #[prop(default = "Enter your content here...".to_string())]
    placeholder: String,
    /// Number of textarea rows
    #[prop(default = 10)]
    rows: u32,
    /// Title for the editor section
    #[prop(default = "Content Editor".to_string())]
    title: String,
    /// Additional CSS classes for the container
    #[prop(default = "".to_string())]
    class: String,
    /// Whether to show content info section
    #[prop(default = false)]
    show_content_info: bool,
    /// Content info to display (e.g., ID, created date)
    #[prop(default = Signal::derive(|| vec![]))]
    content_info: Signal<Vec<(String, String)>>,
) -> impl IntoView {
    // Local state for tracking unsaved changes
    let (has_unsaved_changes, set_has_unsaved_changes) = signal(false);
    let (original_content, set_original_content) = signal(String::new());
    let (is_initialized, set_is_initialized) = signal(false);
    
    // Initialize with current content only once
    Effect::new({
        let content = content.clone();
        let set_original_content = set_original_content.clone();
        let set_is_initialized = set_is_initialized.clone();
        move || {
            if !is_initialized.get() {
                let current = content.get();
                set_original_content.set(current);
                set_is_initialized.set(true);
            }
        }
    });

    // Track content changes to detect unsaved changes
    Effect::new({
        let content = content.clone();
        let original_content = original_content.clone();
        let set_has_unsaved_changes = set_has_unsaved_changes.clone();
        let is_initialized = is_initialized.clone();
        move || {
            if is_initialized.get() {
                let current = content.get();
                let original = original_content.get();
                set_has_unsaved_changes.set(current != original);
            }
        }
    });

    // Handle save trigger
    Effect::new({
        let save_trigger = save_trigger.clone();
        let set_has_unsaved_changes = set_has_unsaved_changes.clone();
        let content = content.clone();
        let set_original_content = set_original_content.clone();
        move || {
            if save_trigger.get() {
                save_trigger.set(false); // Reset trigger
                let current = content.get();
                set_original_content.set(current);
                set_has_unsaved_changes.set(false);
            }
        }
    });

    // Handle cancel trigger
    Effect::new({
        let cancel_trigger = cancel_trigger.clone();
        let content = content.clone();
        let original_content = original_content.clone();
        let set_has_unsaved_changes = set_has_unsaved_changes.clone();
        move || {
            if cancel_trigger.get() {
                cancel_trigger.set(false); // Reset trigger
                let original = original_content.get();
                content.set(original);
                set_has_unsaved_changes.set(false);
            }
        }
    });

    // Determine the current effective status
    let effective_status = Signal::derive({
        let status = status.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        move || {
            match status.get() {
                EditorStatus::Saving => EditorStatus::Saving,
                EditorStatus::Saved => {
                    if has_unsaved_changes.get() {
                        EditorStatus::HasUnsavedChanges
                    } else {
                        EditorStatus::Saved
                    }
                },
                EditorStatus::Idle => {
                    if has_unsaved_changes.get() {
                        EditorStatus::HasUnsavedChanges
                    } else {
                        EditorStatus::Idle
                    }
                },
                EditorStatus::HasUnsavedChanges => EditorStatus::HasUnsavedChanges,
            }
        }
    });

    let container_classes = format!(
        "flex flex-col gap-4 p-6 bg-white rounded-lg shadow-sm border {}",
        class
    );

    let handle_update = move |ev: MouseEvent| {
        ev.prevent_default();
        if has_unsaved_changes.get() {
            // Trigger save operation
            save_trigger.set(true);
        }
    };

    view! {
        <div class=container_classes>
            <div class="flex items-center justify-between">
                <h3 class="text-lg font-semibold text-gray-900">
                    {title}
                </h3>
                <div class="flex items-center gap-2">
                    {move || {
                        match effective_status.get() {
                            EditorStatus::Saving => view! {
                                <div class="flex items-center gap-2 text-sm text-gray-500">
                                    <div class="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
                                    "Saving..."
                                </div>
                            }.into_any(),
                            EditorStatus::HasUnsavedChanges => view! {
                                <div class="flex items-center gap-1 text-sm text-orange-600">
                                    <div class="w-2 h-2 bg-orange-600 rounded-full"></div>
                                    "Unsaved changes"
                                </div>
                            }.into_any(),
                            EditorStatus::Saved => view! {
                                <div class="flex items-center gap-1 text-sm text-green-600">
                                    <div class="w-2 h-2 bg-green-600 rounded-full"></div>
                                    "Saved"
                                </div>
                            }.into_any(),
                            EditorStatus::Idle => view! {}.into_any(),
                        }
                    }}
                </div>
            </div>
            
            // Error display
            {move || {
                if let Some(error_msg) = error.get() {
                    view! {
                        <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-md">
                            <ErrorMessage 
                                message=error_msg 
                                class="".to_string()
                            />
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            
            // Content editor
            <div class="space-y-4">
                <TextArea
                    value=Signal::derive(move || content.get())
                    placeholder=placeholder
                    rows=rows
                    on_input=Box::new({
                        let content = content.clone();
                        move |ev: leptos::ev::Event| {
                            let value = event_target_value(&ev);
                            content.set(value);
                        }
                    })
                    class="min-h-[200px] font-mono text-sm".to_string()
                />
                
                // Action buttons
                <div class="flex justify-between items-center">
                    <div class="text-sm text-gray-500">
                        {move || {
                            match mode.get() {
                                EditorMode::Update => "Updating existing content",
                                EditorMode::Create => "Creating new content",
                            }
                        }}
                    </div>
                    
                    <div class="flex gap-2">
                        {
                            let has_unsaved_changes = has_unsaved_changes.clone();
                            let effective_status = effective_status.clone();
                            let cancel_trigger = cancel_trigger.clone();
                            
                            move || {
                                let cancel_disabled = !has_unsaved_changes.get() || 
                                    matches!(effective_status.get(), EditorStatus::Saving);
                                
                                view! {
                                    <CancelButton
                                        size=ButtonSize::Medium
                                        disabled=cancel_disabled
                                        on_click=Box::new({
                                            let cancel_trigger = cancel_trigger.clone();
                                            move |_| {
                                                cancel_trigger.set(true);
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
                            let effective_status = effective_status.clone();
                            let mode = mode.clone();
                            
                            move || {
                                let save_disabled = !has_unsaved_changes.get() || 
                                    matches!(effective_status.get(), EditorStatus::Saving);
                                
                                let button_text = match mode.get() {
                                    EditorMode::Update => "Update Content",
                                    EditorMode::Create => "Create Content",
                                };
                                
                                view! {
                                    <SPrimaryButton
                                        disabled=save_disabled
                                        on_click=handle_update
                                    >
                                        {button_text}
                                    </SPrimaryButton>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
            
            // Content info
            {move || {
                if show_content_info {
                    let info_items = content_info.get();
                    if !info_items.is_empty() {
                        view! {
                            <div class="pt-4 border-t border-gray-200">
                                <div class="text-xs text-gray-500 space-y-1">
                                    {info_items.into_iter().map(|(label, value)| {
                                        view! {
                                            <div>{label}: " " {value}</div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}
