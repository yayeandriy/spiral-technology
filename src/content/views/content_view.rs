use leptos::prelude::*;

use crate::content::{model::ProjectContent, views::markdown_renderer::MarkdownRenderer};




#[component]
pub fn ContentView(
    content: ProjectContent
) -> impl IntoView {
    
    view! {
        <div class="w-full flex flex-col space-y-2 p-4">            
                {
                    match &content.text {
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
                }            
        </div>
    }
}