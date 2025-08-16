use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{
    content::content_context::use_project_content, 
    ui::{TextEditor, EditorMode, EditorStatus}
};

#[component]
pub fn ProjectContentEditor() -> impl IntoView {
    let content_context = use_project_content();
    
    // Create content signal for two-way binding
    let content_signal = RwSignal::new(String::new());
    
    // Initialize content from context
    Effect::new({
        let content_context = content_context.clone();
        let content_signal = content_signal.clone();
        move || {
            let text = content_context.project_content.0.get()
                .map(|content| content.text.unwrap_or_default())
                .unwrap_or_default();
            content_signal.set(text);
        }
    });
    
    // Create trigger signals for save and cancel operations
    let save_trigger = RwSignal::new(false);
    let cancel_trigger = RwSignal::new(false);
    
    // Handle save trigger
    Effect::new({
        let save_trigger = save_trigger.clone();
        let content_context = content_context.clone();
        let content_signal = content_signal.clone();
        move || {
            if save_trigger.get() {
                let content_context = content_context.clone();
                let text = content_signal.get();
                spawn_local(async move {
                    leptos::logging::log!("About to call create_or_update_project_content");
                    content_context.create_or_update_project_content(Some(text)).await;
                    leptos::logging::log!("create_or_update_project_content completed");
                });
            }
        }
    });
    
    // Derive other signals for the TextEditor component
    let editor_status = Signal::derive({
        let content_context = content_context.clone();
        move || {
            if content_context.is_loading.0.get() {
                EditorStatus::Saving
            } else {
                EditorStatus::Idle
            }
        }
    });
    
    let error_signal = Signal::derive({
        let content_context = content_context.clone();
        move || content_context.error.0.get()
    });
    
    let editor_mode = Signal::derive({
        let content_context = content_context.clone();
        move || {
            if content_context.project_content.0.get().is_some() {
                EditorMode::Update
            } else {
                EditorMode::Create
            }
        }
    });
    
    let content_info = Signal::derive({
        let content_context = content_context.clone();
        move || {
            if let Some(content) = content_context.project_content.0.get() {
                let mut info = vec![
                    ("Content ID".to_string(), content.id.to_string()),
                ];
                
                if let Some(created) = content.created_at.as_ref() {
                    info.push(("Created".to_string(), created.clone()));
                }
                
                info
            } else {
                vec![]
            }
        }
    });

    view! {
        <TextEditor
            content=content_signal
            status=editor_status
            error=error_signal
            mode=editor_mode
            save_trigger=save_trigger
            cancel_trigger=cancel_trigger
            title="Project Content".to_string()
            placeholder="Enter your project content here... This will create new content or update existing content.".to_string()
            rows=10
            show_content_info=true
            content_info=content_info
            show_toolbar=true
            show_settings=false
        />
    }
}