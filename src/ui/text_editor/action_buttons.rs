use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::ui::{
    button::{CancelButton, ButtonSize},
    signal_button::SPrimaryButton,
};

use super::types::{EditorMode, EditorStatus};

#[component]
pub fn ActionButtons(
    /// Editor mode (Create or Update)
    #[prop(into)]
    mode: Signal<EditorMode>,
    /// Current status of the editor
    #[prop(into)]
    status: Signal<EditorStatus>,
    /// Whether there are unsaved changes
    #[prop(into)]
    has_unsaved_changes: Signal<bool>,
    /// Signal to trigger save operation
    #[prop(into)]
    save_trigger: RwSignal<bool>,
    /// Signal to trigger cancel operation
    #[prop(into)]
    cancel_trigger: RwSignal<bool>,
    /// Whether to show mode description
    #[prop(default = true)]
    show_mode_description: bool,
) -> impl IntoView {
    let handle_save = {
        let save_trigger = save_trigger.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        move |ev: MouseEvent| {
            ev.prevent_default();
            if has_unsaved_changes.get() {
                save_trigger.set(true);
            }
        }
    };

    let handle_cancel = {
        let cancel_trigger = cancel_trigger.clone();
        move |_: MouseEvent| {
            cancel_trigger.set(true);
        }
    };

    view! {
        <div class="flex justify-between items-center">
            {move || {
                if show_mode_description {
                    view! {
                        <div class="text-sm text-gray-500">
                            {match mode.get() {
                                EditorMode::Update => "Updating existing content",
                                EditorMode::Create => "Creating new content",
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
            
            <div class="flex gap-2">
                {
                    let has_unsaved_changes = has_unsaved_changes.clone();
                    let status = status.clone();
                    let cancel_trigger = cancel_trigger.clone();
                    
                    move || {
                        let cancel_disabled = !has_unsaved_changes.get() || 
                            matches!(status.get(), EditorStatus::Saving);
                        
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
                    let status = status.clone();
                    let mode = mode.clone();
                    let handle_save = handle_save.clone();
                    
                    move || {
                        let save_disabled = !has_unsaved_changes.get() || 
                            matches!(status.get(), EditorStatus::Saving);
                        
                        let button_text = match mode.get() {
                            EditorMode::Update => "Update Content",
                            EditorMode::Create => "Create Content",
                        };
                        
                        view! {
                            <SPrimaryButton
                                disabled=save_disabled
                                on_click=handle_save
                            >
                                {button_text}
                            </SPrimaryButton>
                        }
                    }
                }
            </div>
        </div>
    }
}
