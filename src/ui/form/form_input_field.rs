use leptos::prelude::*;

use crate::{ shared::data_state_model::DataState, ui::button::PrimaryButton};



#[component]
pub fn InputField<T>(
    data_state: DataState<T>,
    data_handle: impl FnMut() + 'static + Clone + Send, 
    field_name: String
) -> impl IntoView {
    let value = data_state.data.get(&field_name).map(|r| r);

    view! {
        {
            if let Some(value) = value {
                view! {
                    <div class="p-1 rounded-[4px] flex gap-x-1 border border-gray-300">
                                <input
                                    bind:value=*value
                                    placeholder="Enter project title"
                                    type="text" class="p-1 border-none w-full"  />
                                    {
                                        move || {
                                            let mut save_handler = data_handle.clone();
                                            if data_state.is_modified.0.get().contains(&field_name) {
                                                view! { 
                                                      <PrimaryButton 
                                                        on_click=move |_| save_handler()>
                                                        "Save"
                                                    </PrimaryButton>
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

