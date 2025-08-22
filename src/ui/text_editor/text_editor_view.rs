use leptos::prelude::*;

use crate::{ shared::data_state_model::DataState, ui::{button::PrimaryButton, text_editor::{editor_text_area::EditorTextArea, toolbar::Toolbar}}};

#[component]
pub fn TextEditorView<T, P>(
    data_state: DataState<T, P>,
    data_handle: impl FnMut() + 'static + Clone + Send, 
    field_name: String
) -> impl IntoView {
    let value = data_state.data.get(&field_name).cloned();
    view! {
        {
            if let Some(value) = value {
                view! {
                    <Toolbar 
                    data_state=data_state
                    data_handle=data_handle.clone()
                    field_name=field_name.clone()
                    />
                    <EditorTextArea
                        value=value.clone()
                    />
                }.into_any()
            } else {
               view!{<div/>}.into_any()
            }
        }

                            

    }
}

