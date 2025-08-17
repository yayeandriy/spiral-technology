use leptos::{logging, prelude::*};
use leptos::task::spawn_local;
use std::collections::HashMap;

use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::projects::views::project_edit_page::form_input_field::InputField;
use crate::projects::views::project_edit_page::form_text_area::FormTextArea;

#[derive(Clone)]
pub struct DataState<T> {
    pub data: HashMap<String, (ReadSignal<String>, WriteSignal<String>)>,
    pub is_modified: (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>),
    pub id: i32,
    pub created_at: String,
    init_data: Option<T>,
}

impl DataState<Project> {
    pub fn new(input_data: Option<Project>) -> Self {
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: input_data.as_ref().map_or(0, |p| p.id),
            created_at: input_data.as_ref().map_or(String::new(), |p| p.created_at.clone().unwrap_or_default()),
            init_data: input_data,
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

    fn get_init_value(&self, field_name: &str) -> String {
        if let Some(project) = &self.init_data {
            match field_name {
                "title" => project.title.clone(),
                "desc" => project.desc.clone().unwrap_or_default(),
                _ => String::new(),
            }
        } else {
            String::new()
        }
    }

    pub fn check_modified(&self) {
        logging::log!("Checking if project is modified...");
        
        // Clear the modified list
        self.is_modified.1.set(vec![]);
        
        // Iterate through all fields in the data HashMap
        for (field_name, (read_signal, _)) in &self.data {
            let current_value = read_signal.get();
            let init_value = self.get_init_value(field_name);
            
            logging::log!("Field '{}' - current: '{}', initial: '{}'", field_name, current_value, init_value);
            
            if current_value != init_value {
                let field_name_clone = field_name.clone();
                self.is_modified.1.update(|v| {
                    if !v.contains(&field_name_clone) {
                        v.push(field_name_clone);
                    }
                });
            }
        }
    }
}#[component]
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
    let project_state_clone_desc = project_state.clone();
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
    let handle_save_project_clone_desc = handle_save_project.clone();
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
                            <FormTextArea
                                data_state=project_state_clone_desc
                                data_handle=handle_save_project_clone_desc
                                field_name="desc".to_string()
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

