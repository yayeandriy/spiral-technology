use leptos::prelude::*;

use crate::{ shared::data_state_model::DataState, ui::{button::PrimaryButton, text_editor::toolbar::Toolbar}};

#[component]
pub fn EditorTextArea(
    value: (ReadSignal<String>, WriteSignal<String>)
) -> impl IntoView {
    view! {
    
                    <div class="p-1 rounded-[4px] w-full flex h-full border border-gray-300">
                               <textarea
                                        class="p-1 border-none w-full resize-none"
                                        prop:value=move || value.0.get()
                                        on:input:target=move |ev| value.1.set(ev.target().value())
                                    >
                                        {value.0.get()}
                                    </textarea>
                                  
                                  
                            </div>
        
    }
}

