use leptos::prelude::*;
use crate::{
    shared::data_state_model::{DataState, DataHandler}, 
    ui::text_editor::{editor_text_area::EditorTextArea, toolbar::Toolbar},
    content::views::markdown_renderer::MarkdownRenderer
};

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Edit,
    Preview,
    Split,
}

#[component]
pub fn NotionStyleEditor<T, P>(
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler,
{
    let value = data_state.data.get(&field_name).cloned();
    let view_mode = RwSignal::new(ViewMode::Split);
    let is_editing = RwSignal::new(false);
    
    view! {
        {
            if let Some(value) = value {
                let markdown_text = move || value.0.get();
                
                view! {
                    <div class="w-full h-screen flex flex-col bg-white">
                        // Floating toolbar
                        <div class="sticky top-0 z-10 bg-white border-b shadow-sm">
                            <div class="flex justify-between items-center p-3">
                                <div class="flex gap-2">
                                    <button 
                                        class=move || format!("px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                            if view_mode.get() == ViewMode::Edit { 
                                                "bg-blue-100 text-blue-700 border border-blue-200" 
                                            } else { 
                                                "bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                            }
                                        )
                                        on:click=move |_| view_mode.set(ViewMode::Edit)
                                    >
                                        "✏️ Edit"
                                    </button>
                                    <button 
                                        class=move || format!("px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                            if view_mode.get() == ViewMode::Preview { 
                                                "bg-blue-100 text-blue-700 border border-blue-200" 
                                            } else { 
                                                "bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                            }
                                        )
                                        on:click=move |_| view_mode.set(ViewMode::Preview)
                                    >
                                        "👁️ Preview"
                                    </button>
                                    <button 
                                        class=move || format!("px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                            if view_mode.get() == ViewMode::Split { 
                                                "bg-blue-100 text-blue-700 border border-blue-200" 
                                            } else { 
                                                "bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                            }
                                        )
                                        on:click=move |_| view_mode.set(ViewMode::Split)
                                    >
                                        "📑 Split"
                                    </button>
                                </div>
                                
                                // Save indicator
                                {
                                    let data_state_save = data_state.clone();
                                    let field_name_save = field_name.clone();
                                    move || {
                                        let data_state_save = data_state_save.clone();
                                        if data_state_save.is_modified.0.get().contains(&field_name_save) {
                                            view! {
                                                <div class="flex items-center gap-2">
                                                    <span class="text-sm text-orange-600">"Unsaved changes"</span>
                                                    <button 
                                                        class="px-3 py-1.5 text-sm bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
                                                        on:click=move |_| data_state_save.update_or_create()
                                                    >
                                                        "Save"
                                                    </button>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <span class="text-sm text-green-600">"✓ Saved"</span>
                                            }.into_any()
                                        }
                                    }
                                }
                            </div>
                        </div>
                        
                        // Main content area
                        <div class="flex-1 flex overflow-hidden">
                            {
                                let data_state_clone = data_state.clone();
                                let field_name_clone = field_name.clone();
                                let value_clone = value.clone();
                                let markdown_text_clone = markdown_text.clone();
                                move || {
                                    let markdown_text = markdown_text_clone.clone();
                                    match view_mode.get() {
                                    ViewMode::Edit => {
                                        // Full editor view with advanced features
                                        view! {
                                            <div class="w-full h-full">
                                                <EditorTextArea value=value_clone.clone()>
                                                    <div class="p-4 border-b bg-gray-50">
                                                        <Toolbar 
                                                            data_state=data_state_clone.clone()
                                                            field_name=field_name_clone.clone()
                                                        />
                                                    </div>
                                                </EditorTextArea>
                                            </div>
                                        }.into_any()
                                    },
                                    ViewMode::Preview => {
                                        // Full preview view
                                        view! {
                                            <div class="w-full overflow-y-auto">
                                                <div class="max-w-4xl mx-auto p-8">
                                                    <div 
                                                        class="min-h-screen cursor-text"
                                                        on:click=move |_| {
                                                            view_mode.set(ViewMode::Edit);
                                                        }
                                                    >
                                                        {
                                                            let markdown_text_inner = markdown_text.clone();
                                                            move || {
                                                                let text = markdown_text_inner();
                                                                if text.trim().is_empty() {
                                                                    view! {
                                                                        <div class="text-gray-400 text-lg">
                                                                            "Click to start writing..."
                                                                        </div>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {
                                                                        <MarkdownRenderer text=text />
                                                                    }.into_any()
                                                                }
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_any()
                                    },
                                    ViewMode::Split => {
                                        // Split view with advanced editor
                                        view! {
                                            <div class="w-full flex h-full">
                                                // Editor side with full context
                                                <div class="w-1/2 border-r">
                                                    <EditorTextArea value=value_clone.clone()>
                                                        <div class="p-3 border-b bg-gray-50 text-sm font-medium text-gray-700">
                                                            "📝 Markdown Editor"
                                                        </div>
                                                    </EditorTextArea>
                                                </div>
                                                
                                                // Preview side
                                                <div class="w-1/2 flex flex-col bg-gray-50">
                                                    <div class="p-3 border-b bg-gray-100 text-sm font-medium text-gray-700">
                                                        "👁️ Live Preview"
                                                    </div>
                                                    <div class="flex-1 p-6 overflow-y-auto bg-white">
                                                        {
                                                            let markdown_text_inner2 = markdown_text.clone();
                                                            move || {
                                                                let text = markdown_text_inner2();
                                                                if text.trim().is_empty() {
                                                                    view! {
                                                                        <div class="text-gray-400">
                                                                            "Your markdown will appear here..."
                                                                        </div>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {
                                                                        <MarkdownRenderer text=text />
                                                                    }.into_any()
                                                                }
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                }
                            }}
                        </div>
                    </div>
                }.into_any()
            } else {
                view!{<div class="w-full h-screen flex items-center justify-center text-gray-500">"No content available"</div>}.into_any()
            }
        }
    }
}
