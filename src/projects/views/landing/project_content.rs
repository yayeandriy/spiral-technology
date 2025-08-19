use leptos::prelude::*;

use crate::{content::model::ProjectContent, projects::views::landing::markdown_renderer::MarkdownRenderer};




#[component]
pub fn ProjectContentView(
    content: ProjectContent
) -> impl IntoView {
    
    view! {
        <div class="project-content">
            <div class="markdown-content prose prose-lg max-w-none">
                {
                    match &content.text {
                        Some(text) => {
                            view! {
                                // <div inner_html=text.clone()></div>
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
        </div>
    }
}