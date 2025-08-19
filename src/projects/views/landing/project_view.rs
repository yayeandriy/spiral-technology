use leptos::prelude::*;

use crate::{content::{self, content_context::use_project_content}, projects::{projects_context::use_project, views::landing::{project_content::ProjectContentView, project_header::ProjectHeader}}};




#[component]
pub fn ProjectView() -> impl IntoView {
    let project_context = use_project();
    let content_context = use_project_content();
    let project = move || project_context.get_current_project();
    let content = move || content_context.project_content.0.get();

    view! {
        <div class="flex flex-col gap-2 mt-20">
            {
                move || if let Some(project) = project() {
                    view! {
                        <ProjectHeader project=project />
                    }.into_any()
                }else{
                    view!{<div/>}.into_any()
                }
            }
            {
                move || if let Some(content) = content() {
                    view! {
                        <ProjectContentView content=content />
                    }.into_any()
                }else{
                    view!{<div/>}.into_any()
                }
            }
        </div>
    }
}