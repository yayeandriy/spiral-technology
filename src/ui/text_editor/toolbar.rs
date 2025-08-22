use leptos::prelude::*;

use crate::{ 
    shared::data_state_model::{DataState, DataHandler}, 
    ui::{button::PrimaryButton, text_editor::editor_text_area::MarkdownEditor}
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
    
    view! {
        <div class="w-full h-[40px]  bg-gray-100 justify-between border-b flex items-center rounded mb-1">
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
        </div>
         {
            let data_state = data_state.clone();
            let field_name = field_name.clone();
            move || {
                let data_state = data_state.clone();
                if data_state.is_modified.0.get().contains(&field_name) {
                    view! { 
                        <div class="" >
                            <PrimaryButton
                            on_click=move |_| data_state.update_or_create()>
                            "Save"
                            </PrimaryButton>
                        </div>
                        }.into_any()
                } else {
                    view! { <div />}.into_any()
                }
            }
        }
          
        </div>
    
    }
      
}

