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
    
    view! {
        {
            if let Some(value) = value {
                view! {
                    <div class="w-full h-screen flex flex-col bg-white">
                        // Top bar with mode toggles and save
                        <div class="sticky top-0 z-10 bg-white border-b shadow-sm">
                            <div class="flex justify-between items-center p-3">
                                <div class="flex gap-2">
                                    <button 
                                        class=move || if view_mode.get() == ViewMode::Edit { 
                                            "px-3 py-1.5 text-sm rounded-md bg-blue-100 text-blue-700 border border-blue-200" 
                                        } else { 
                                            "px-3 py-1.5 text-sm rounded-md bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                        }
                                        on:click=move |_| view_mode.set(ViewMode::Edit)
                                    >
                                        "✏️ Edit"
                                    </button>
                                    <button 
                                        class=move || if view_mode.get() == ViewMode::Preview { 
                                            "px-3 py-1.5 text-sm rounded-md bg-blue-100 text-blue-700 border border-blue-200" 
                                        } else { 
                                            "px-3 py-1.5 text-sm rounded-md bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                        }
                                        on:click=move |_| view_mode.set(ViewMode::Preview)
                                    >
                                        "👁️ Preview"
                                    </button>
                                    <button 
                                        class=move || if view_mode.get() == ViewMode::Split { 
                                            "px-3 py-1.5 text-sm rounded-md bg-blue-100 text-blue-700 border border-blue-200" 
                                        } else { 
                                            "px-3 py-1.5 text-sm rounded-md bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                        }
                                        on:click=move |_| view_mode.set(ViewMode::Split)
                                    >
                                        "📑 Split"
                                    </button>
                                </div>
                                
                                // Save indicator
                                <SaveIndicator data_state=data_state.clone() field_name=field_name.clone() />
                            </div>
                        </div>
                        
                        // Content area
                        <div class="flex-1 overflow-hidden">
                            <Show when=move || view_mode.get() == ViewMode::Edit>
                                <EditView value=value.clone() data_state=data_state.clone() field_name=field_name.clone() />
                            </Show>
                            
                            <Show when=move || view_mode.get() == ViewMode::Preview>
                                <PreviewView value=value.clone() view_mode=view_mode />
                            </Show>
                            
                            <Show when=move || view_mode.get() == ViewMode::Split>
                                <SplitView value=value.clone() data_state=data_state.clone() field_name=field_name.clone() />
                            </Show>
                        </div>
                    </div>
                }.into_any()
            } else {
                view!{<div class="w-full h-screen flex items-center justify-center text-gray-500">"No content available"</div>}.into_any()
            }
        }
    }
}

#[component]
fn SaveIndicator<T, P>(
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler,
{
    let data_state_save = data_state.clone();
    let field_name_save = field_name.clone();
    
    view! {
        {move || {
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
        }}
    }
}

#[component]
fn EditView<T, P>(
    value: (ReadSignal<String>, WriteSignal<String>),
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler,
{
    view! {
        <div class="w-full h-full">
            <EditorTextArea value=value>
                <div class="p-4 border-b bg-gray-50">
                    <Toolbar data_state=data_state field_name=field_name />
                </div>
            </EditorTextArea>
        </div>
    }
}

#[component]
fn PreviewView(
    value: (ReadSignal<String>, WriteSignal<String>),
    view_mode: RwSignal<ViewMode>
) -> impl IntoView {
    view! {
        <div class="w-full h-full overflow-y-auto">
            <div class="max-w-4xl mx-auto p-8">
                <div 
                    class="min-h-screen cursor-text"
                    on:click=move |_| view_mode.set(ViewMode::Edit)
                >
                    {move || {
                        let text = value.0.get();
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
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn SplitView<T, P>(
    value: (ReadSignal<String>, WriteSignal<String>),
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler,
{
    view! {
        <div class="w-full h-full flex">
            // Editor side
            <div class="w-1/2 border-r">
                <EditorTextArea value=value.clone()>
                    <div class="p-3 border-b bg-gray-50 text-sm font-medium text-gray-700 flex justify-between">
                        <span>"📝 Markdown Editor"</span>
                        <Toolbar data_state=data_state field_name=field_name />
                    </div>
                </EditorTextArea>
            </div>
            
            // Preview side
            <div class="w-1/2 flex flex-col bg-gray-50">
                <div class="p-3 border-b bg-gray-100 text-sm font-medium text-gray-700">
                    "👁️ Live Preview"
                </div>
                <div class="flex-1 p-6 overflow-y-auto bg-white">
                    {move || {
                        let text = value.0.get();
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
                    }}
                </div>
            </div>
        </div>
    }
}
