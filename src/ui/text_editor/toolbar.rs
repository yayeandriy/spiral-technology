use leptos::prelude::*;

use crate::{ shared::data_state_model::DataState, ui::button::PrimaryButton};

#[component]
pub fn Toolbar<T, P>(
    data_state: DataState<T, P>,
    data_handle: impl FnMut() + 'static + Clone + Send, 
    field_name: String
) -> impl IntoView {
    
    view! {
        <div class="w-full h-[40px]  bg-gray-100 justify-between border-b flex items-center rounded mb-1">
        <div class="">
            Toolbar
        </div>
         {
            move || {
                let mut save_handler = data_handle.clone();
                if data_state.is_modified.0.get().contains(&field_name) {
                    view! { 
                        <div class="" >
                            <PrimaryButton 
                            on_click=move |_| save_handler()>
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

