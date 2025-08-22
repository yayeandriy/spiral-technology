use leptos::prelude::*;

use crate::{ shared::data_state_model::{DataState, DataHandler, MarkdownHandler}, ui::button::PrimaryButton};

#[component]
pub fn Toolbar<T, P>(
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler + MarkdownHandler,
{
    
    view! {
        <div class="w-full h-[40px]  bg-gray-100 justify-between border-b flex items-center rounded mb-1">
        <div class="flex gap-x-2 px-2">
            <button 
                class="px-2 py-1 text-sm bg-white border rounded hover:bg-gray-50"
                on:click={
                    let data_state = data_state.clone();
                    let field_name = field_name.clone();
                    move |_| {
                        // Get current text and make it bold
                        if let Some((read_signal, write_signal)) = data_state.data.get(&field_name) {
                            let current_text = read_signal.get();
                            let bold_text = data_state.make_bold(&current_text);
                            write_signal.set(bold_text);
                        }
                    }
                }>
                "B"
            </button>
            <button 
                class="px-2 py-1 text-sm bg-white border rounded hover:bg-gray-50 italic"
                on:click={
                    let data_state = data_state.clone();
                    let field_name = field_name.clone();
                    move |_| {
                        // Get current text and make it italic
                        if let Some((read_signal, write_signal)) = data_state.data.get(&field_name) {
                            let current_text = read_signal.get();
                            let italic_text = data_state.make_italic(&current_text);
                            write_signal.set(italic_text);
                        }
                    }
                }>
                "I"
            </button>
            <button 
                class="px-2 py-1 text-sm bg-white border rounded hover:bg-gray-50"
                on:click={
                    let data_state = data_state.clone();
                    let field_name = field_name.clone();
                    move |_| {
                        // Get current text and make it H1
                        if let Some((read_signal, write_signal)) = data_state.data.get(&field_name) {
                            let current_text = read_signal.get();
                            let h1_text = data_state.make_h1(&current_text);
                            write_signal.set(h1_text);
                        }
                    }
                }>
                "H1"
            </button>
            <button 
                class="px-2 py-1 text-sm bg-white border rounded hover:bg-gray-50"
                on:click={
                    let data_state = data_state.clone();
                    let field_name = field_name.clone();
                    move |_| {
                        // Get current text and make it H2
                        if let Some((read_signal, write_signal)) = data_state.data.get(&field_name) {
                            let current_text = read_signal.get();
                            let h2_text = data_state.make_h2(&current_text);
                            write_signal.set(h2_text);
                        }
                    }
                }>
                "H2"
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

