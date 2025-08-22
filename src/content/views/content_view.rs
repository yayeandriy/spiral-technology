use leptos::prelude::*;

use crate::content::{content_context::use_project_content, views::markdown_renderer::MarkdownRenderer};




#[component]
pub fn ContentView() -> impl IntoView {
    let content_context = use_project_content();
    let content = move || content_context.project_content.0.get();
    
    view! {
        <div class="w-full bg-white pt-4 flex flex-col space-y-2 p-4 pt-4">            
            {
                move || if let Some(content_data) = content() {
                    match &content_data.text {
                        Some(text) => {
                            view! {
                                <MarkdownRenderer text=text.clone() />
                            }.into_any()
                        },
                        None => {
                            view! {
                                <div class="text-gray-500 italic">
                                    "No content available"
                                </div>
                            }.into_any()
                        }
                    }
                } else {
                    view! {
                        <div/>
                    }.into_any()
                }
            }           
        </div>
    }
}