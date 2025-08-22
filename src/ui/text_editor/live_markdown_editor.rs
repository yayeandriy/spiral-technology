use leptos::prelude::*;
use crate::{
    shared::data_state_model::{DataState, DataHandler}, 
    ui::text_editor::{editor_text_area::EditorTextArea, toolbar::Toolbar},
    content::views::markdown_renderer::MarkdownRenderer
};

#[component]
pub fn LiveMarkdownEditor<T, P>(
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler,
{
    let value = data_state.data.get(&field_name).cloned();
    let show_split_view = RwSignal::new(true);
    
    view! {
        {
            if let Some(value) = value {
                let markdown_text = move || value.0.get();
                
                view! {
                    <div class="w-full h-screen flex flex-col">
                        // Editor area with toolbar inside EditorTextArea context
                        <div class="flex-1 flex overflow-hidden">
                            {move || {
                                if show_split_view.get() {
                                    // Split view: editor on left, preview on right
                                    view! {
                                        <div class="w-1/2 border-r flex flex-col">
                                            <EditorTextArea value=value.clone()>
                                                <div class="flex justify-between items-center p-2 bg-gray-50 border-b">
                                                    <Toolbar 
                                                        data_state=data_state.clone()
                                                        field_name=field_name.clone()
                                                    />
                                                    
                                                    <div class="flex gap-2">
                                                        <button 
                                                            class="px-3 py-1 text-sm rounded border bg-white"
                                                            on:click=move |_| show_split_view.set(false)
                                                        >
                                                            "Raw"
                                                        </button>
                                                        <button 
                                                            class="px-3 py-1 text-sm rounded border bg-blue-500 text-white"
                                                            on:click=move |_| show_split_view.set(true)
                                                        >
                                                            "Split"
                                                        </button>
                                                    </div>
                                                </div>
                                            </EditorTextArea>
                                        </div>
                                        <div class="w-1/2 p-4 overflow-y-auto bg-white">
                                            <MarkdownRenderer text=markdown_text() />
                                        </div>
                                    }.into_any()
                                } else {
                                    // Raw editor only
                                    view! {
                                        <div class="w-full flex flex-col">
                                            <EditorTextArea value=value.clone()>
                                                <div class="flex justify-between items-center p-2 bg-gray-50 border-b">
                                                    <Toolbar 
                                                        data_state=data_state.clone()
                                                        field_name=field_name.clone()
                                                    />
                                                    
                                                    <div class="flex gap-2">
                                                        <button 
                                                            class="px-3 py-1 text-sm rounded border bg-blue-500 text-white"
                                                            on:click=move |_| show_split_view.set(false)
                                                        >
                                                            "Raw"
                                                        </button>
                                                        <button 
                                                            class="px-3 py-1 text-sm rounded border bg-white"
                                                            on:click=move |_| show_split_view.set(true)
                                                        >
                                                            "Split"
                                                        </button>
                                                    </div>
                                                </div>
                                            </EditorTextArea>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                }.into_any()
            } else {
                view!{<div/>}.into_any()
            }
        }
    }
}
