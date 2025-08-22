use leptos::prelude::*;

use crate::{ shared::data_state_model::{DataState, DataHandler}, ui::button::PrimaryButton};

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
    
    view! {
        <div class="w-full h-[40px]  bg-gray-100 justify-between border-b flex items-center rounded mb-1">
        <div class="">
            Toolbar
        </div>
         {
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

