use leptos::prelude::*;

use crate::{projects::views::project_edit_page::project_form::DataState, ui::signal_button::SPrimaryButton};



#[component]
pub fn FormTextArea<T>(
    data_state: DataState<T>,
    data_handle: impl FnMut() + 'static + Clone + Send, 
    field_name: String
) -> impl IntoView {
    let value = data_state.data.get(&field_name).cloned();

    view! {
        {
            if let Some(value) = value {
                view! {
                    <div class="p-1 rounded-[4px] w-full h-full flex gap-x-1 border border-gray-300">
                               <textarea
                                        class="p-1 border-none w-full resize-none"
                                        prop:value=move || value.0.get()
                                        on:input:target=move |ev| value.1.set(ev.target().value())
                                    >
                                        {value.0.get()}
                                    </textarea>
                                    {
                                        move || {
                                            let mut save_handler = data_handle.clone();
                                            if data_state.is_modified.0.get().contains(&field_name) {
                                                view! { 
                                                    <div class="h-20" >
                                                      <SPrimaryButton 
                                                        on_click=move |_| save_handler()>
                                                        "Save"
                                                        </SPrimaryButton>
                                                    </div>
                                                    }.into_any()
                                            } else {
                                                view! { <div />}.into_any()
                                            }
                                        }
                                    }
                                  
                            </div>}.into_any()
            } else {
               view!{<div/>}.into_any()
            }
        }

                            

    }
}

