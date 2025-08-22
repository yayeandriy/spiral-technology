use std::{collections::HashMap, sync::Arc};

use leptos::{prelude::*, reactive::spawn_local};

use crate::{content::{content_context::{use_project_content, ProjectContentContext}, model::ProjectContent}, shared::data_state_model::{DataState, DataHandler}, ui::text_editor::text_editor_view::TextEditorView};


impl DataHandler for DataState<ProjectContent,ProjectContentContext> {
    fn update_or_create(&self) {
        let context = self.context.clone();
        let state = Arc::new(self.clone());
        if let Some(context) = context {
            let update_content = Arc::try_unwrap(state)
                        .unwrap_or_else(|arc| (*arc).clone())
                        .into_data();
             spawn_local(async move {
                    let updated_content = update_content.clone();
                    let text = updated_content.text.clone();
                    context.create_or_update_project_content(text).await;
            });
        } else {
            return;
        }
    }
}

impl DataState<ProjectContent,ProjectContentContext> {
    pub fn new() -> Self {
        Self {           

            ..Default::default()
        }
    }
    pub fn from_data(input_data: Option<ProjectContent>) -> Self {
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: input_data.as_ref().map_or(0, |p| p.id as i32),
            created_at: input_data.as_ref().map_or(String::new(), |p| p.created_at.clone().unwrap_or_default()),
            init_data: input_data,
            ..Default::default()    
        }
    }
    pub fn from_data_with_context(input_data: Option<ProjectContent>, context: Arc<ProjectContentContext>) -> Self {
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: input_data.as_ref().map_or(0, |p| p.id as i32),
            created_at: input_data.as_ref().map_or(String::new(), |p| p.created_at.clone().unwrap_or_default()),
            init_data: input_data,
            context: Some(context),
            ..Default::default()
        }
    }

    pub fn init_fields(&mut self) {
        if let Some(project_content) = &self.init_data {
            self.data.insert("text".to_string(), signal(project_content.text.clone().unwrap_or_default()));
        } else {
            self.data.insert("text".to_string(), signal(String::new()));
        }
    }

    pub fn into_data(self) -> ProjectContent {
        ProjectContent {
            text: self.data.get("text").map(|(r, _)| r.get()),
            id: self.id as i64,
            project_id: self.init_data.as_ref().map_or(0, |p| p.project_id),
            created_at: Some(self.created_at),
        }
    }

    fn get_init_value(&self, field_name: &str) -> String {
        if let Some(project_content) = &self.init_data {
            match field_name {
                "text" => project_content.text.clone().unwrap_or_default(),
                _ => String::new(),
            }
        } else {
            String::new()
        }
    }

    pub fn check_modified(&self) {
        
        // Clear the modified list
        self.is_modified.1.set(vec![]);
        
        // Iterate through all fields in the data HashMap
        for (field_name, (read_signal, _)) in &self.data {
            let current_value = read_signal.get();
            let init_value = self.get_init_value(field_name);
            
            
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

    pub fn update_or_create(&self) {
        let context = self.context.clone();
        let state = Arc::new(self.clone());
        if let Some(context) = context {
            let update_content = Arc::try_unwrap(state)
                        .unwrap_or_else(|arc| (*arc).clone())
                        .into_data();
             spawn_local(async move {
                    let updated_content = update_content.clone();
                    let text = updated_content.text.clone();
                    context.create_or_update_project_content(text).await;
            });
        } else {
            return;
        }
    }
}


#[component]
pub fn ContentEditor(
    content: ProjectContent
) -> impl IntoView {
    let context = use_project_content();
   let content_clone = content.clone();
   let mut content_state = DataState::<ProjectContent, ProjectContentContext>::from_data_with_context(Some(content_clone), context.clone());

   content_state.init_fields();
   content_state.listen_for_changes();

    let project_content_state_clone = Arc::new(content_state.clone()); 

    view! {
        <div class="text-black h-[600px] ">
            <TextEditorView 
                data_state=(*project_content_state_clone).clone()
                field_name="text".to_string()
            />
        </div>
    }
}
