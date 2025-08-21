use leptos::{logging, prelude::*};
use leptos::task::spawn_local;
use std::collections::HashMap;
use std::sync::Arc;

use crate::content::views::content_page::ContentPage;
use crate::projects::projects_context::use_project;
use crate::projects::model::Project;
use crate::projects::views::editor::project_edit_page::project_areas_editor::ProjectAreasEditor;
use crate::shared::data_state_model::DataState;
use crate::ui::button::DangerButton;
use crate::ui::form::form_input_field::InputField;
use crate::ui::form::form_text_area::FormTextArea;
use crate::ui::tabs::Tabs;


impl DataState<Project> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: 0,
            created_at: String::new(),
            init_data: None,
        }
    }
    pub fn from_data(input_data: Option<Project>) -> Self {
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
            self.data.insert("order".to_string(), signal(project.order.unwrap_or_default().to_string()));
        } else {
            self.data.insert("title".to_string(), signal(String::new()));
            self.data.insert("desc".to_string(), signal(String::new()));
            self.data.insert("order".to_string(), signal(String::new()));
        }
    }

    pub fn into_data(self) -> Project {
        Project {
            title: self.data.get("title").map(|(r, _)| r.get()).unwrap_or_default(),
            desc: Some(self.data.get("desc").map(|(r, _)| r.get()).unwrap_or_default()),
            order: self.data.get("order").map(|(r, _)| r.get()).and_then(|s| s.parse().ok()),
            id: self.id,
            created_at: Some(self.created_at),
        }
    }

    fn get_init_value(&self, field_name: &str) -> String {
        if let Some(project) = &self.init_data {
            match field_name {
                "title" => project.title.clone(),
                "desc" => project.desc.clone().unwrap_or_default(),
                "order" => project.order.unwrap_or_default().to_string(),
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

    pub fn listen_for_changes(&self) {
        let s_clone = self.clone();
        Effect::new(move || {
            s_clone.check_modified();
        });
    }

}

#[component]
pub fn ProjectForm(
    #[prop(optional)] project: Option<Project>,
) -> impl IntoView {
    let project_context = use_project();
    let mut project_state = if let Some(project) = project.clone() {
        DataState::<Project>::from_data(Some(project))
    } else {
        DataState::<Project>::new()
    };
    project_state.init_fields();
    project_state.listen_for_changes();

    let project_state_clone = Arc::new(project_state.clone());

    let handle_save_project = {
        let project_context = project_context.clone();
        let project_state = project_state.clone();
        move || {
            logging::log!("Saving project...");
            let project_context = project_context.clone();
            let project_state = project_state.clone();
            spawn_local(async move {
                    let updated_project = project_state.into_data();
                    project_context.update_project(updated_project).await;
            });
        }
    };
    let navigate = leptos_router::hooks::use_navigate();

    let handle_create_project = {
        let project_context = project_context.clone();
        let project_state = project_state.clone();
        let navigate = navigate.clone();
        move || {
            logging::log!("Creating project...");
            let project_context = project_context.clone();
            let project_state = project_state.clone();
            let navigate = navigate.clone();
            spawn_local(async move {
                    let updated_project = project_state.into_data();
                    let created_project = project_context.add_project(updated_project).await;
                    if let Some(created_project) = created_project {
                        navigate(&format!("/editor/{}", created_project.id), Default::default());
                    }
            });
        }
    };

    let handle_delete_project = {
        let project_context = project_context.clone();
        move |project_id: i32| {
            logging::log!("Deleting project with ID: {}", project_id);
             let project_context = project_context.clone();
            spawn_local(async move {
                project_context.delete_project(project_id).await;
                navigate("/editor", Default::default());
            });
        }
    };

    let handle_save_project_clone = Arc::new(handle_save_project.clone());
    let handle_create_project_clone = Arc::new(handle_create_project.clone());


    view! {
        <div class="p-6 bg-white text-black w-full h-screen flex flex-col">
            <div class="flex space-x-4">
               {
                    if let Some(project_id) = project.as_ref().map(|p| p.id) {
                        let handle_delete_project = handle_delete_project.clone();
                        view!{
                        <div class="w-1/2 flex flex-col space-y-4">
                            <DangerButton 
                            on_click=move |_| {
                                let handle_delete_project = handle_delete_project.clone();
                                handle_delete_project(project_id);
                            }
                            >Delete project</DangerButton>
                            <InputField
                                data_state=(*project_state_clone).clone()
                                data_handle=(*handle_save_project_clone).clone()
                                field_name="title".to_string()
                            />
                            <InputField
                                data_state=(*project_state_clone).clone()
                                data_handle=(*handle_save_project_clone).clone()
                                field_name="order".to_string()
                            />
                            <FormTextArea
                                data_state=(*project_state_clone).clone()
                                data_handle=(*handle_save_project_clone).clone()
                                field_name="desc".to_string()
                            />
                            <Tabs
                                tabs_titles=vec!["Content".into(), "Areas".into()]
                            >
                                <ContentPage />
                                <ProjectAreasEditor
                                    project_id=project_id
                                    />
                            </Tabs>
                        </div>
                      
                                }.into_any()
                    }else{
                        view!{ <div class="w-1/2 flex flex-col space-y-4">
                            <InputField
                                data_state=(*project_state_clone).clone()
                                data_handle=(*handle_create_project_clone).clone()
                                field_name="title".to_string()
                            />
                            <FormTextArea
                                data_state=(*project_state_clone).clone()
                                data_handle=(*handle_create_project_clone).clone()
                                field_name="desc".to_string()
                            />                           
                        </div>}.into_any()
                    }
                }
               
        </div>
        </div>
    }
}

