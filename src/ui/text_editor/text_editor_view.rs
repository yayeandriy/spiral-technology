use leptos::prelude::*;

use crate::{ shared::data_state_model::{DataState, DataHandler, MarkdownHandler}, ui::text_editor::{editor_text_area::EditorTextArea, toolbar::Toolbar}};

#[component]
pub fn TextEditorView<T, P>(
    data_state: DataState<T, P>,
    field_name: String
) -> impl IntoView 
where
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    DataState<T, P>: DataHandler + MarkdownHandler,
{
    let value = data_state.data.get(&field_name).cloned();
    view! {
        {
            if let Some(value) = value {
                view! {
                    <Toolbar 
                    data_state=data_state
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

