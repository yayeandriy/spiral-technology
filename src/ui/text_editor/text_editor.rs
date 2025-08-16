use leptos::prelude::*;

use crate::ui::{textarea::TextArea, error::ErrorMessage};

use super::{
    types::{EditorMode, EditorStatus},
    status_indicator::StatusIndicator,
    action_buttons::ActionButtons,
    content_info::ContentInfo,
    toolbar::{EditorToolbar, ToolbarAction},
    settings::{EditorSettings, EditorConfig},
};

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
    /// Whether to show action buttons
    #[prop(default = true)]
    show_action_buttons: bool,
    /// Whether to show mode description in action buttons
    #[prop(default = true)]
    show_mode_description: bool,
    /// Whether to show the toolbar
    #[prop(default = false)]
    show_toolbar: bool,
    /// Whether to show editor settings
    #[prop(default = false)]
    show_settings: bool,
    /// Editor configuration
    #[prop(default = RwSignal::new(EditorConfig::default()))]
    editor_config: RwSignal<EditorConfig>,
) -> impl IntoView {
    // Local state for tracking unsaved changes
    let (has_unsaved_changes, set_has_unsaved_changes) = signal(false);
    let (original_content, set_original_content) = signal(String::new());
    let (is_initialized, set_is_initialized) = signal(false);
    let (is_fullscreen, set_is_fullscreen) = signal(false);
    
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

    // Handle toolbar actions
    let handle_toolbar_action = Callback::new({
        let content = content.clone();
        let set_is_fullscreen = set_is_fullscreen.clone();
        move |action: ToolbarAction| {
            match action {
                ToolbarAction::Bold => {
                    // Simple bold formatting - wrap selection or add **text**
                    let current = content.get();
                    content.set(format!("{}**bold text**", current));
                },
                ToolbarAction::Italic => {
                    let current = content.get();
                    content.set(format!("{}*italic text*", current));
                },
                ToolbarAction::Code => {
                    let current = content.get();
                    content.set(format!("{}`code`", current));
                },
                ToolbarAction::List => {
                    let current = content.get();
                    content.set(format!("{}\n- List item", current));
                },
                ToolbarAction::Quote => {
                    let current = content.get();
                    content.set(format!("{}\n> Quote", current));
                },
                ToolbarAction::Link => {
                    let current = content.get();
                    content.set(format!("{}[Link text](url)", current));
                },
                ToolbarAction::FullScreen => {
                    set_is_fullscreen.update(|fs| *fs = !*fs);
                },
                _ => {
                    // TODO: Implement other actions like Undo, Redo, Find, Replace
                }
            }
        }
    });

    let container_classes = if is_fullscreen.get() {
        format!(
            "fixed inset-0 z-50 flex flex-col bg-white {}",
            class
        )
    } else {
        format!(
            "flex flex-col gap-4 p-6 bg-white rounded-lg shadow-sm border {}",
            class
        )
    };

    view! {
        <div class=move || container_classes.clone()>
            <div class="flex items-center justify-between">
                <h3 class="text-lg font-semibold text-gray-900">
                   {title}
                </h3>
                <StatusIndicator
                    status=status
                    has_unsaved_changes=Signal::derive(move || has_unsaved_changes.get())
                />
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

            // Toolbar
            <EditorToolbar
                show_toolbar=show_toolbar
                on_action=handle_toolbar_action
                is_fullscreen=is_fullscreen.get()
            />
            
            // Content editor
            <div class="space-y-4 flex-1">
                <TextArea
                    value=Signal::derive(move || content.get())
                    placeholder=placeholder
                    rows=if is_fullscreen.get() { 25 } else { rows }
                    on_input=Box::new({
                        let content = content.clone();
                        move |ev: leptos::ev::Event| {
                            let value = event_target_value(&ev);
                            content.set(value);
                        }
                    })
                    class=format!(
                        "min-h-[200px] font-mono {}",
                        if is_fullscreen.get() { "h-full" } else { "text-sm" }
                    )
                />
                
                // Action buttons
                {move || {
                    if show_action_buttons {
                        view! {
                            <ActionButtons
                                mode=mode
                                status=status
                                has_unsaved_changes=Signal::derive(move || has_unsaved_changes.get())
                                save_trigger=save_trigger
                                cancel_trigger=cancel_trigger
                                show_mode_description=show_mode_description
                            />
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
            
            // Content info
            <ContentInfo
                show_content_info=show_content_info
                content_info=content_info
            />

            // Settings
            <EditorSettings
                config=editor_config
                show_settings=show_settings
            />
        </div>
    }
}
