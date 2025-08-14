use leptos::prelude::*;

use crate::projects::views::{project_form::ProjectFormPage, projects_list::ProjectsList};




#[component]
pub fn ProjectsEditor() -> impl IntoView {
  
    view! {
        <ProjectFormPage />
    }
}