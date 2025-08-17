use leptos::{logging, prelude::*};

use crate::{projects::views::project_edit_page::project_form::{ModifiedData, DataState}, ui::signal_button::SPrimaryButton};



#[component]
pub fn InputField<T>(
    data_state: DataState<T>,
    data_handle: impl FnMut() + 'static + Clone + Send, 
    field_name: String
) -> impl IntoView {
    let project_state_clone = data_state.clone();
    let value = project_state_clone.data.get(&field_name).map(|r| r);

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
                                            if project_state_clone.is_modified.0.get().contains(&ModifiedData::Title) {
                                                view! { 
                                                      <SPrimaryButton 
                                                        on_click=move |_| save_handler()>
                                                        "Save"
                                                    </SPrimaryButton>
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

