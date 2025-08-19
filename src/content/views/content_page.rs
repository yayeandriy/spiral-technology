use leptos::{html::S, prelude::*, reactive::spawn_local};

use crate::{content::{content_context::use_project_content, views::content_editor::ContentEditor}, ui::PrimaryButton};




#[component]
pub fn ContentPage() -> impl IntoView {
    let content_context = use_project_content();
    let content_context_clone = content_context.clone();
    let content_context_clone_2 = content_context.clone();
    let content =move || content_context_clone.project_content.0.get().clone();

    let handle_create_new_content = move || {
        spawn_local(async move {
            // Assuming create_new_project_content is a method that creates a new content
            // and updates the context accordingly.
            content_context_clone_2.create_or_update_project_content(None).await;
        });
        
    };
    let handle_create_new_content = handle_create_new_content.clone();

    view! {
        <div class="text-black ">
        {
            move || {
               
                if let Some(content) = content() {
                    view! {
                       <ContentEditor content=content />
                    }.into_any()
                } else {
                    let handle_create_new_content_clone = handle_create_new_content.clone();
                    view! {
                        <div class="">
                            <PrimaryButton on_click=move |_| handle_create_new_content_clone.clone()()>
                                "Create New Content"
                            </PrimaryButton>
                        </div>
                    }.into_any()
                }
            }
        }
            
        </div>
    }
}
