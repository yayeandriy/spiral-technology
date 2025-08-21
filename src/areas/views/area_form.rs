use std::{collections::HashMap, sync::Arc};

use leptos::{logging, prelude::*, reactive::spawn_local};

use crate::{areas::{areas_context::use_areas, model::ProjectArea}, catalog::catalog_context::use_catalog, shared::data_state_model::DataState, ui::{button::{ButtonSize, CancelButton, DangerButton}, form::{form_input_field::InputField, form_text_area::FormTextArea}}};




impl DataState<ProjectArea> {
    pub fn new(input_data: Option<ProjectArea>) -> Self {
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: input_data.as_ref().map_or(0, |p| p.id as i32),
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

    pub fn into_data(self) -> ProjectArea{
        ProjectArea {
            title: self.data.get("title").map(|(r, _)| r.get()).unwrap_or_default(),
            desc: Some(self.data.get("desc").map(|(r, _)| r.get()).unwrap_or_default()),
            id: self.id as i64,
            created_at: Some(self.created_at),
            category: self.init_data.as_ref().map_or("no category".to_string(), |p| p.category.clone()),
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

    pub fn listen_for_changes(&self) {
        let s_clone = self.clone();
        Effect::new(move || {
            s_clone.check_modified();
        });
    }

    pub fn from_category(category: String) -> Self {
        let init_data = ProjectArea {
            id: 0,
            title: String::new(),
            desc: None,
            created_at: Some(String::new()),
            category,
        };
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: 0,
            created_at: String::new(),
            init_data: Some(init_data),
        }
    }

}




#[component]
pub fn AreaForm(
    #[prop(optional)]
    area: Option<ProjectArea>,
    category: String,
    is_open: WriteSignal<bool>,
) -> impl IntoView {
    let catalog_context = use_catalog();
    let areas_context = use_areas();
    let areas_context_clone = areas_context.clone();

    let area_clone = area.clone();

    let mut area_state = if let Some(area) = area {
        DataState::<ProjectArea>::new(Some(area))
    } else {
        DataState::<ProjectArea>::from_category(category)
    };
    
    
    area_state.init_fields();
    area_state.listen_for_changes();

    let area_state_clone = Arc::new(area_state.clone());
    let area_state_clone_2 = Arc::new(area_state.clone());
    let area_state_clone_3 = Arc::new(area_state.clone());

   
    let handle_create_area = {
        let areas_context = areas_context_clone.clone();
        let area_state_clone = area_state_clone_3.clone();
        move || {
            logging::log!("Saving project...");
            let areas_context = areas_context.clone();
            let area_state = area_state_clone.clone();
            spawn_local(async move {
                    let updated_area = <DataState<ProjectArea> as Clone>::clone(&area_state).into_data();
                    areas_context.create_area(updated_area).await;
            });
        }
    };
   
    let handle_update_area = {
        let areas_context = areas_context_clone.clone();
        let area_state_clone = area_state_clone_3.clone();
        move || {
            logging::log!("Saving project...");
            let areas_context = areas_context.clone();
            let area_state = area_state_clone.clone();
            spawn_local(async move {
                    let updated_area = <DataState<ProjectArea> as Clone>::clone(&area_state).into_data();
                    areas_context.update_area(updated_area).await;
            });
        }
    };
    let handle_delete_area = {
        let areas_context = areas_context_clone.clone();
        let area_state_clone = area_state_clone_3.clone();
        move || {
            logging::log!("Saving project...");
            let areas_context = areas_context.clone();
            let area_state = area_state_clone.clone();
            spawn_local(async move {
                    let updated_area = <DataState<ProjectArea> as Clone>::clone(&area_state).into_data();
                    
                    areas_context.delete_area(updated_area.id).await;
            });
            is_open.set(false);
        }
    };

    let handle_create_area_clone = Arc::new(handle_create_area.clone());
    let handle_update_area_clone = Arc::new(handle_update_area.clone());

    view! {
        <div class="">
            <div class="w-full flex flex-col space-y-4">
            {
                if area_clone.is_some() {
                    view! {
                         <InputField
                        data_state=(*area_state_clone).clone()
                        data_handle=(*handle_update_area_clone).clone()
                        field_name="title".to_string()
                    />
                    <FormTextArea
                        data_state=(*area_state_clone).clone()
                        data_handle=(*handle_update_area_clone).clone()
                        field_name="desc".to_string()
                    />             
                    }.into_any()
                } else {
                    view! {
                        <InputField
                        data_state=(*area_state_clone).clone()
                        data_handle=(*handle_create_area_clone).clone()
                        field_name="title".to_string()
                    />
                    <FormTextArea
                        data_state=(*area_state_clone).clone()
                        data_handle=(*handle_create_area_clone).clone()
                        field_name="desc".to_string()
                    />             
                    }.into_any()
                }
            }
                                 
                </div>
                <div class="flex justify-between mt-1">
                    {
                        if let Some(area) = area_clone {
                            if area.id > 0 {
                                view!{
                                     <DangerButton
                                        size=ButtonSize::Small
                                        on_click=move |_| {
                                            logging::log!("Canceling area edit...");
                                            handle_delete_area();                                                                                    

                                        }
                                        >"🗑️"</DangerButton>
                                }.into_any( )
                            } else {
                                view!{<div class="grow" />}.into_any()
                            }
                        }else{
                            view!{<div class="grow" />}.into_any()
                        }
                    }
                   
                    
                    <CancelButton 
                    size=ButtonSize::Small
                    on_click=move |_| {
                        logging::log!("Canceling area edit...");
                        is_open.set(false);
                    }
                    >"╳"</CancelButton>
                    
                </div>
        </div>
                           
    }
}

