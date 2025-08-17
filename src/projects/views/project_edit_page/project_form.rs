use leptos::{logging, prelude::*};
use leptos::html::Input;
use leptos::task::spawn_local;
use web_sys::SubmitEvent;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::areas::areas_context::use_areas;
use crate::areas::model::ProjectArea;
use crate::catalog::catalog_context::use_catalog;
use crate::projects::views::project_content_editor::ProjectContentEditor;
use crate::projects::views::project_edit_page::input_field::InputField;
use crate::projects::views::project_edit_page::project_form_notifications::{AutoSaveStatus, ProjectFormNotifications};
use crate::ui::signal_button::SPrimaryButton;
use crate::ui::*;


#[derive(Clone, PartialEq)]
pub enum ModifiedData {
    Title,
    Description,
}

#[derive(Clone)]
pub struct DataState<T> {
    pub data: HashMap<String, (ReadSignal<String>, WriteSignal<String>)>,
    pub is_modified: (ReadSignal<Vec<ModifiedData>>, WriteSignal<Vec<ModifiedData>>),
    pub id: i32,
    pub created_at: String,
    init_data: Option<T>,
}

impl DataState<Project> {
    pub fn new(project: Option<Project>) -> Self {

        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: project.as_ref().map_or(0, |p| p.id),
            created_at: project.as_ref().map_or(String::new(), |p| p.created_at.clone().unwrap_or_default()),
            init_data: project,
        }
    }   
    pub fn init_fields(&mut self) {
        if let Some(project) = &self.init_data {
            self.data.insert("title".to_string(), signal(project.title.clone()));
            self.data.insert("desc".to_string(), signal(project.desc.clone().unwrap_or_default()));
        } else {
            self.data.insert("title".to_string(), signal(String::new()));
            self.data.insert("desc".to_string(), signal(String::new()));
        }
    }

    pub fn into_project(self) -> Project {
        Project {
            title: self.data.get("title").map(|(r, _)| r.get()).unwrap_or_default(),
            desc: Some(self.data.get("desc").map(|(r, _)| r.get()).unwrap_or_default()),
            id: self.id,
            created_at: Some(self.created_at),
        }
    }

    pub fn check_modified(&self) {
        logging::log!("Checking if project is modified...");
        let new_title = self.data.get("title").map(|(r, _)| r.get()).unwrap_or_default();
        let init_title = self.init_data.as_ref().map_or(String::new(), |p| p.title.clone());
        logging::log!("Project title changed to: {}", new_title);
        self.is_modified.1.set(vec![]);
        if new_title != init_title {
            self.is_modified.1.update(|v| {
                if !v.contains(&ModifiedData::Title) {
                    v.push(ModifiedData::Title);
                }
            });
        }
        let new_desc = self.data.get("desc").map(|(r, _)| r.get()).unwrap_or_default();
        let init_desc = self.init_data.as_ref().and_then(|p| p.desc.clone()).unwrap_or_default();
        if new_desc != init_desc {
            self.is_modified.1.update(|v| {
                if !v.contains(&ModifiedData::Description) {
                    v.push(ModifiedData::Description);
                }
            });
        } 
        
    }

}

#[component]
pub fn ProjectForm(
    #[prop(optional)] project: Option<Project>,
) -> impl IntoView {
    let project_context = use_project();
    let mut project_state = DataState::new(project.clone());
    project_state.init_fields();

    let project_state_clone = project_state.clone();
    Effect::new(move || {
        project_state_clone.check_modified(); 
    });
    let project_state_clone = project_state.clone();
    let handle_save_project = {
        let project_context = project_context.clone();
        move || {
            logging::log!("Saving project...");
            let project_context = project_context.clone();
            let project_state = project_state.clone();
            spawn_local(async move {
                    let updated_project = project_state.into_project();
                    project_context.update_project(updated_project).await;
            });
        }
    };
    let handle_save_project_clone = handle_save_project.clone();
       view! {
        <div class="p-6 bg-white text-black w-full h-screen flex flex-col">
           
            
            <div class="w-full flex flex-col space-y-6">
               
                // Top section with title/desc on left, areas on right
                <div class="flex gap-8 flex-1">
                    // Left column - Title and Description
                    <div class="w-1/2 space-y-6">
                        // Title field
                        <div class="space-y-2">
                            <InputField
                                data_state=project_state_clone
                                data_handle=handle_save_project_clone
                                field_name="title".to_string()
                            />
                            // <div class="p-1 rounded-[4px] flex gap-x-1 border border-gray-300">
                            //     <input
                            //         bind:value=project_state_clone.title
                            //         placeholder="Enter project title"
                            //         type="text" class="p-1 border-none w-full"  />
                            //         {
                            //             move || {
                            //                 let save_handler = handle_save_project.clone();
                            //                 if project_state_clone.is_modified.0.get().contains(&ModifiedData::Title) {
                            //                     view! { 
                            //                           <SPrimaryButton 
                            //                             on_click=save_handler>
                            //                             "Save"
                            //                         </SPrimaryButton>
                            //                         }.into_any()
                            //                 } else {
                            //                     view! { <div />}.into_any()
                            //                 }
                            //             }
                            //         }
                                  
                            // </div>

                        </div>
                            // <div class="p-1 rounded-[4px] flex gap-x-1 border border-gray-300">
                            //    <textarea
                            //             prop:value=move || project_state_clone_3.desc.0.get()
                            //             on:input:target=move |ev| project_state_clone.desc.1.set(ev.target().value())
                            //         >
                            //             {project_state_clone_2.desc.0.get()}
                            //         </textarea>
                            //         {
                            //             move || {
                            //                 let save_handler = handle_save_project_clone.clone();
                            //                 if project_state_clone.is_modified.0.get().contains(&ModifiedData::Description) {
                            //                     view! { 
                            //                           <SPrimaryButton 
                            //                             on_click=save_handler>
                            //                             "Save"
                            //                         </SPrimaryButton>
                            //                         }.into_any()
                            //                 } else {
                            //                     view! { <div />}.into_any()
                            //                 }
                            //             }
                            //         }
                                  
                            // </div>
                        
                    </div>
                </div>
            </div>
        </div>
    }
}

