use leptos::prelude::*;

use crate::{ 
    shared::data_state_model::{DataState, DataHandler}, 
    ui::{button::PrimaryButton, text_editor::{editor_text_area::MarkdownEditor, markdown_renderer::MarkdownRenderer}}
};

#[component]
pub fn Toolbar<T, P>(
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler,
{
    let markdown_editor = use_context::<MarkdownEditor>().expect("MarkdownEditor context not found");
    let show_preview = RwSignal::new(false);
    
    // Clone for closures
    let data_state_for_preview = data_state.clone();
    let field_name_for_preview = field_name.clone();
    let data_state_for_save = data_state.clone();
    let field_name_for_save = field_name.clone();
    
    // Get the current text content for preview
    let preview_text = move || {
        if let Some(value) = data_state_for_preview.data.get(&field_name_for_preview) {
            value.0.get()
        } else {
            String::new()
        }
    };
    
    view! {
        <div>
            <div class="w-full h-[40px] bg-gray-100 justify-between border-b flex items-center rounded mb-1">
                <div class="flex gap-x-2 px-2">
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let apply_bold = markdown_editor.apply_bold.clone();
                            move |_| {
                                apply_bold();
                            }
                        }
                    >
                        "Bold"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let apply_italic = markdown_editor.apply_italic.clone();
                            move |_| {
                                apply_italic();
                            }
                        }
                    >
                        "Italic"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let apply_h1 = markdown_editor.apply_h1.clone();
                            move |_| {
                                apply_h1();
                            }
                        }
                    >
                        "H1"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let apply_h2 = markdown_editor.apply_h2.clone();
                            move |_| {
                                apply_h2();
                            }
                        }
                    >
                        "H2"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let insert_link = markdown_editor.insert_link.clone();
                            move |_| {
                                insert_link();
                            }
                        }
                    >
                        "Link"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let apply_quote = markdown_editor.apply_quote.clone();
                            move |_| {
                                apply_quote();
                            }
                        }
                    >
                        "Quote"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100"
                        on:click={
                            let insert_image = markdown_editor.insert_image.clone();
                            move |_| {
                                insert_image();
                            }
                        }
                    >
                        "Image"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100 bg-yellow-50"
                        on:click={
                            let undo = markdown_editor.undo.clone();
                            move |_| {
                                undo();
                            }
                        }
                    >
                        "↶ Undo"
                    </button>
                    
                    <button 
                        class="flex items-center gap-1 px-2 py-1 text-sm border rounded hover:bg-gray-100 bg-blue-50"
                        on:click=move |_| {
                            show_preview.set(!show_preview.get());
                        }
                    >
                        "Preview"
                    </button>
                </div>
                
                {
                    let data_state_save = data_state_for_save.clone();
                    let field_name_save = field_name_for_save.clone();
                    move || {
                        let data_state_save = data_state_save.clone();
                        if data_state_save.is_modified.0.get().contains(&field_name_save) {
                            view! { 
                                <div class="">
                                    <PrimaryButton
                                        on_click=move |_| data_state_save.update_or_create()
                                    >
                                        "Save"
                                    </PrimaryButton>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div/> }.into_any()
                        }
                    }
                }
            </div>
            
            // Preview Modal
            {move || {
                if show_preview.get() {
                    view! {
                        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                            <div class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4 flex flex-col">
                                <div class="flex justify-between items-center p-4 border-b">
                                    <h2 class="text-lg font-semibold">"Markdown Preview"</h2>
                                    <button 
                                        class="text-gray-500 hover:text-gray-700 text-2xl w-8 h-8 flex items-center justify-center"
                                        on:click=move |_| show_preview.set(false)
                                    >
                                        "×"
                                    </button>
                                </div>
                                <div class="p-4 overflow-y-auto flex-1">
                                    <MarkdownRenderer text=preview_text() />
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div/> }.into_any()
                }
            }}
        </div>
    }
}
