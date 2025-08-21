use crate::areas::model::{ProjectArea};
use crate::supabase::{supabase_get, supabase_post, supabase_patch, supabase_delete};
use leptos::{
    logging,
    prelude::{
        provide_context,
        signal,
        use_context,
        Children,
        Effect,
        ReadSignal,
        Set,
        Update,
        WriteSignal,
    },
    task::spawn_local,
    *,
};
use std::sync::Arc;
use leptos::prelude::Get;



#[derive(Clone)]
pub struct AreaContext {
    pub areas: (ReadSignal<Vec<ProjectArea>>, WriteSignal<Vec<ProjectArea>>),
    pub categories: (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>),
    pub default_category: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    url_path: String,
}

impl AreaContext {
    pub fn new() -> Self {
        Self {
            areas: signal::<Vec<ProjectArea>>(vec![]),
            categories: signal::<Vec<String>>(vec![]),
            default_category: signal(None),
            is_loading: signal(false),
            error: signal(None),
            url_path: "/rest/v1/areas_view?select=*".to_string(),
        }
    }

    pub async fn fetch_areas(&self) {
        self.is_loading.1.try_update(|v| {
            *v = true;
        });
        self.error.1.update(|e| {
            *e = None;
        });
        match supabase_get::<Vec<ProjectArea>>(&self.url_path).await {
            Ok(items) => {
                logging::log!("Fetched areas successfully: {:?}", items);
                self.parse_categories(items.clone());
                self.areas.1.set(items);
            }
            Err(err) => {
                logging::log!("Error fetching areas: {}", err);
            }
        }
    }

    fn parse_categories(&self, items: Vec<ProjectArea>) {
        let mut categories: Vec<String> = items
            .into_iter()
            .map(|item| item.category)
            .collect();
        categories.sort();
        categories.dedup();
        self.categories.1.set(categories);
        let first_category = self.categories.0.get().first().cloned();
        self.default_category.1.set(first_category);
        logging::log!("Parsed categories: {:?}", self.categories.0.get());
    }

    pub async fn add_area(&self, title: String, category: String, desc: Option<String>) {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        
        let new_area = serde_json::json!({
            "title": title,
            "category": category,
            "desc": desc
        });
        
        match supabase_post::<ProjectArea, serde_json::Value>("/rest/v1/areas", &new_area).await {
            Ok(area) => {
                self.areas.1.update(|areas| {
                    areas.push(area);
                });
                // Refresh categories
                let current_areas = self.areas.0.get();
                self.parse_categories(current_areas);
            }
            Err(err) => {
                logging::log!("Error adding area: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }

    pub async fn create_area(&self, area: ProjectArea) {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        
        let new_area = serde_json::json!({
            "title": area.title,
            "category": area.category,
            "desc": area.desc
        });

        logging::log!("Creating area with data: {:?}", new_area);   
        
        match supabase_post::<ProjectArea, serde_json::Value>("/rest/v1/areas", &new_area).await {
            Ok(area) => {
                self.areas.1.update(|areas| {
                    areas.push(area);
                });
                // Refresh categories
                let current_areas = self.areas.0.get();
                self.parse_categories(current_areas);
            }
            Err(err) => {
                logging::log!("Error adding area: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }

    pub async fn update_area(&self, area: ProjectArea) {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        
        let updated_area = serde_json::json!({
            "title": area.title,
            "category": area.category,
            "desc": area.desc,
            "order": area.order
        });
        
        match supabase_patch::<ProjectArea, serde_json::Value>(&format!("/rest/v1/areas?id=eq.{}", area.id), &updated_area).await {
            Ok(updated) => {
                self.areas.1.update(|areas| {
                    if let Some(pos) = areas.iter().position(|a| a.id == area.id) {
                        areas[pos] = updated;
                    }
                });
                // Refresh categories
                let current_areas = self.areas.0.get();
                self.parse_categories(current_areas);
            }
            Err(err) => {
                logging::log!("Error updating area: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }

    pub async fn delete_area(&self, area_id: i64) {
        self.is_loading.1.try_update(|v| *v = true);
        self.error.1.update(|e| *e = None);
        
        match supabase_delete(&format!("/rest/v1/areas?id=eq.{}", area_id)).await {
            Ok(_) => {
                self.areas.1.update(|areas| {
                    areas.retain(|area| area.id != area_id);
                });
                // Refresh categories
                let current_areas = self.areas.0.get();
                self.parse_categories(current_areas);
            }
            Err(err) => {
                logging::log!("Error deleting area: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.try_update(|v| *v = false);
    }

    pub fn get_areas_by_category(&self, category: &String) -> Vec<ProjectArea> {
        self.areas.0.get().iter()
            .filter(|area| &area.category == category)
            .cloned()
            .collect()
    }
    pub fn get_areas_by_ids(&self, ids: &[i64]) -> Vec<ProjectArea> {
        self.areas.0.get().iter()
            .filter(|area| ids.contains(&area.id))
            .cloned()
            .collect()
    }

}

#[component]
pub fn AreaContextProvider(children: Children) -> impl IntoView {
    let editor_context = Arc::new(AreaContext::new());
   
    provide_context(editor_context);

    children()
}

#[component]
pub fn AreaRoute(children: Children) -> impl IntoView {
    let area_context = use_areas();
    spawn_local(async move {
        area_context.fetch_areas().await;
    });

    children()
}


pub fn use_areas() -> Arc<AreaContext> {
    use_context::<Arc<AreaContext>>().expect(
        "AreaContext not found. Make sure you are using AreaContextProvider."
    )
}
