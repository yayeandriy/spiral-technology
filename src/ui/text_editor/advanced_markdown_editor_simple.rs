use leptos::prelude::*;

use crate::ui::textarea::TextArea;
use super::{
    types::{ViewMode, MarkdownConfig, EditorMode, EditorStatus},
    toolbar::{EditorToolbar, ToolbarAction},
    markdown_processor::{MarkdownProcessor, MarkdownPreview},
};

#[component]
pub fn AdvancedMarkdownEditor(
    /// Text content signal for read and write
    #[prop(into)]
    content: RwSignal<String>,
    /// Error message, if any
    #[prop(into)]
    error: Signal<Option<String>>,
    /// Placeholder text for the textarea
    #[prop(default = "# 1Start writing your Markdown content...\n\nUse the toolbar above to format your text.".to_string())]
    placeholder: String,
    /// Number of textarea rows
    #[prop(default = 20)]
    rows: u32,
    /// Title for the editor section
    #[prop(default = "Advanced Markdown Editor".to_string())]
    title: String,
    /// Optional save callback
    #[prop(optional)]
    on_save: Option<Callback<String>>,
    /// Optional cancel callback  
    #[prop(optional)]
    on_cancel: Option<Callback<()>>,
    /// Editor mode (Create/Update)
    #[prop(default = Signal::derive(|| EditorMode::Create))]
    mode: Signal<EditorMode>,
    /// Editor status
    #[prop(default = Signal::derive(|| EditorStatus::Idle))]
    status: Signal<EditorStatus>,
) -> impl IntoView {
    // Local state for UI - make view_mode reactive
    let view_mode = RwSignal::new(ViewMode::Edit);
    let (is_fullscreen, set_is_fullscreen) = signal(false);
    let (search_visible, set_search_visible) = signal(false);
    let (search_term, set_search_term) = signal(String::new());
    
    // Markdown configuration
    let markdown_config = MarkdownConfig::default();
    
    // Textarea reference for programmatic access
    let textarea_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();
    
        // Handle toolbar actions
    let on_toolbar_action = move |action: ToolbarAction| {
        match action {
            ToolbarAction::Preview => {
                view_mode.set(ViewMode::Preview);
            }
            ToolbarAction::SplitView => {
                view_mode.set(ViewMode::Split);
            }
            ToolbarAction::FullScreen => {
                set_is_fullscreen.set(!is_fullscreen.get());
            }
            ToolbarAction::Find => {
                set_search_visible.set(true);
            }
            ToolbarAction::Bold => {
                insert_bold(&content);
            }
            ToolbarAction::Italic => {
                insert_italic(&content);
            }
            ToolbarAction::Header1 => {
                insert_header(&content, 1);
            }
            ToolbarAction::Header2 => {
                insert_header(&content, 2);
            }
            ToolbarAction::Header3 => {
                insert_header(&content, 3);
            }
            ToolbarAction::BulletList => {
                insert_list(&content, "- ");
            }
            ToolbarAction::NumberedList => {
                insert_list(&content, "1. ");
            }
            ToolbarAction::Link => {
                insert_link(&content);
            }
            _ => {
                // Handle other actions as needed
            }
        }
    };

    let container_class = format!(
        "bg-white rounded-lg shadow-sm border border-gray-200 {}",
        if is_fullscreen.get() { "fixed inset-0 z-50 rounded-none border-0" } else { "" }
    );

    // Calculate metrics for display
    let processor = MarkdownProcessor::new(markdown_config.clone());
    let metrics = processor.calculate_metrics(&content.get());

    view! {
        <div class=container_class>
            // Header with title and metrics
            <div class="flex flex-col sm:flex-row sm:items-center justify-between p-4 border-b border-gray-200 gap-2">
                <h3 class="text-lg font-semibold text-gray-900">{title}</h3>
                <div class="flex items-center space-x-2 sm:space-x-4 text-xs sm:text-sm text-gray-600">
                    <span>{format!("Words: {}", metrics.word_count)}</span>
                    <span>{format!("Lines: {}", metrics.line_count)}</span>
                    <span class="hidden sm:inline">{format!("~{} min read", metrics.reading_time_minutes)}</span>
                </div>
            </div>

            // Toolbar
            <EditorToolbar 
                show_toolbar=true
                on_action=Callback::new(on_toolbar_action)
                is_fullscreen=is_fullscreen.get()
            />

            // View mode selector
            <div class="bg-gray-50 border-b border-gray-200 px-2 sm:px-4 py-2">
                <div class="flex items-center justify-center sm:justify-start space-x-1 sm:space-x-2">
                    <button
                        on:click=move |_| view_mode.set(ViewMode::Edit)
                        class=format!("px-2 sm:px-3 py-1 text-xs sm:text-sm rounded {}", 
                            if matches!(view_mode.get(), ViewMode::Edit) { 
                                "bg-blue-100 text-blue-700" 
                            } else { 
                                "text-gray-600 hover:bg-gray-100" 
                            }
                        )
                    >
                        <span class="sm:hidden">"📝"</span>
                        <span class="hidden sm:inline">"📝 Edit"</span>
                    </button>
                    <button
                        on:click=move |_| view_mode.set(ViewMode::Preview)
                        class=format!("px-2 sm:px-3 py-1 text-xs sm:text-sm rounded {}", 
                            if matches!(view_mode.get(), ViewMode::Preview) { 
                                "bg-blue-100 text-blue-700" 
                            } else { 
                                "text-gray-600 hover:bg-gray-100" 
                            }
                        )
                    >
                        <span class="sm:hidden">"👁"</span>
                        <span class="hidden sm:inline">"👁 Preview"</span>
                    </button>
                    <button
                        on:click=move |_| view_mode.set(ViewMode::Split)
                        class=format!("px-2 sm:px-3 py-1 text-xs sm:text-sm rounded {}", 
                            if matches!(view_mode.get(), ViewMode::Split) { 
                                "bg-blue-100 text-blue-700" 
                            } else { 
                                "text-gray-600 hover:bg-gray-100" 
                            }
                        )
                    >
                        <span class="sm:hidden">"⫸"</span>
                        <span class="hidden sm:inline">"⫸ Split"</span>
                    </button>
                </div>
            </div>

            // Search bar (if visible)
            {if search_visible.get() {
                view! {
                    <div class="bg-yellow-50 border-b border-yellow-200 p-3">
                        <div class="flex items-center space-x-2">
                            <input 
                                type="text"
                                placeholder="Search..."
                                value=search_term.get()
                                on:input=move |ev| set_search_term.set(event_target_value(&ev))
                                class="flex-1 px-3 py-1 text-sm border border-gray-300 rounded focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <button 
                                on:click=move |_| set_search_visible.set(false)
                                class="px-2 py-1 text-sm text-gray-600 hover:text-gray-900"
                            >
                                "✕"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            // Main editor area
            <div class="flex min-h-96 flex-col lg:flex-row">
                {match view_mode.get() {
                    ViewMode::Edit => view! {
                        <div class="w-full">
                            <div class="p-2 sm:p-4">
                                <TextArea
                                    value=content
                                    placeholder=placeholder
                                    rows=rows
                                    node_ref=textarea_ref
                                    class="w-full min-h-64 sm:min-h-80 lg:min-h-96 font-mono text-sm leading-relaxed resize-none border-0 focus:ring-0 p-0".to_string()
                                />
                            </div>
                        </div>
                    }.into_any(),
                    ViewMode::Preview => view! {
                        <div class="w-full">
                            <MarkdownPreview 
                                content=content.get()
                                config=markdown_config.clone()
                            />
                        </div>
                    }.into_any(),
                    ViewMode::Split => view! {
                        <div class="w-full lg:w-1/2 border-b lg:border-b-0 lg:border-r border-gray-200">
                            <div class="p-2 sm:p-4">
                                <TextArea
                                    value=content
                                    placeholder=placeholder
                                    rows=rows
                                    node_ref=textarea_ref
                                    class="w-full min-h-32 sm:min-h-48 lg:min-h-96 font-mono text-sm leading-relaxed resize-none border-0 focus:ring-0 p-0".to_string()
                                />
                            </div>
                        </div>
                        <div class="w-full lg:w-1/2">
                            <MarkdownPreview 
                                content=content.get()
                                config=markdown_config.clone()
                            />
                        </div>
                    }.into_any(),
                }}
            </div>

            // Action buttons
            {if on_save.is_some() || on_cancel.is_some() {
                view! {
                    <div class="p-4 bg-gray-50 border-t border-gray-200 flex items-center justify-between">
                        <div class="flex items-center space-x-2 text-sm text-gray-600">
                            <span>{format!("Status: {:?}", status.get())}</span>
                            <span>{format!("Mode: {:?}", mode.get())}</span>
                        </div>
                        <div class="flex items-center space-x-2">
                            {if let Some(cancel_cb) = on_cancel {
                                view! {
                                    <button
                                        on:click=move |_| cancel_cb.run(())
                                        class="px-4 py-2 text-sm text-gray-600 border border-gray-300 rounded hover:bg-gray-100"
                                    >
                                        "Cancel"
                                    </button>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }}
                            
                            {if let Some(save_cb) = on_save {
                                let content_clone = content;
                                view! {
                                    <button
                                        on:click=move |_| save_cb.run(content_clone.get())
                                        disabled=matches!(status.get(), EditorStatus::Saving)
                                        class="px-4 py-2 text-sm text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                                    >
                                        {match mode.get() {
                                            EditorMode::Create => "Create Content",
                                            EditorMode::Update => "Update Content",
                                            _ => "Save Content"
                                        }}
                                    </button>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}

            // Error display
            {match error.get() {
                Some(err) => view! {
                    <div class="p-4 bg-red-50 border-t border-red-200">
                        <div class="text-sm text-red-600">{err}</div>
                    </div>
                }.into_any(),
                None => view! {}.into_any(),
            }}
        </div>
    }
}

// Helper functions for markdown insertion using pure string manipulation
fn insert_bold(content: &RwSignal<String>) {
    let current = content.get();
    let new_content = format!("{}**bold text**", current);
    content.set(new_content);
}

fn insert_italic(content: &RwSignal<String>) {
    let current = content.get();
    let new_content = format!("{}*italic text*", current);
    content.set(new_content);
}

fn insert_header(content: &RwSignal<String>, level: u8) {
    let current = content.get();
    let header_prefix = "#".repeat(level as usize);
    let new_content = if current.is_empty() {
        format!("{} Header Text", header_prefix)
    } else {
        format!("{}\n{} Header Text", current, header_prefix)
    };
    content.set(new_content);
}

fn insert_list(content: &RwSignal<String>, prefix: &str) {
    let current = content.get();
    let new_content = if current.is_empty() {
        format!("{}List item", prefix)
    } else {
        format!("{}\n{}List item", current, prefix)
    };
    content.set(new_content);
}

fn insert_link(content: &RwSignal<String>) {
    let current = content.get();
    let new_content = format!("{}[link text](url)", current);
    content.set(new_content);
}
