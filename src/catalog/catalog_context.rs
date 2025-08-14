use crate::catalog::model::{ProjectAreaLink};
use crate::supabase::{supabase_get, supabase_post, supabase_delete};
use leptos::{
    logging,
    prelude::{
        provide_context,
        signal,
        use_context,
        Children,
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
pub struct CatalogContext {
    pub catalog: (ReadSignal<Vec<ProjectAreaLink>>, WriteSignal<Vec<ProjectAreaLink>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    url_path: String,
}

impl CatalogContext {
    pub fn new() -> Self {
        Self {
            catalog: signal::<Vec<ProjectAreaLink>>(vec![]),
            is_loading: signal(false),
            error: signal(None),
            url_path: "/rest/v1/catalog?select=*".to_string(),
        }
    }

    pub async fn fetch_catalog(&self) {
        self.is_loading.1.try_update(|v| {
            *v = true;
        });
        self.error.1.update(|e| {
            *e = None;
        });
        match supabase_get::<Vec<ProjectAreaLink>>(&self.url_path).await {
            Ok(items) => {
                logging::log!("Fetched catalog successfully: {:?}", items);
                self.catalog.1.set(items);
            }
            Err(err) => {
                logging::log!("Error fetching catalog: {}", err);
            }
        }
    }

    pub async fn add_project_area_relation(&self, project_id: i64, area_id: i64) -> Result<(), String> {
        self.is_loading.1.try_update(|v| *v = true);
        
        let relation_data = serde_json::json!({
            "project_id": project_id,
            "area_id": area_id
        });
        
        match supabase_post::<ProjectAreaLink, serde_json::Value>("/rest/v1/catalog", &relation_data).await {
            Ok(relation) => {
                self.catalog.1.update(|catalog| {
                    catalog.push(relation);
                });
                self.is_loading.1.try_update(|v| *v = false);
                Ok(())
            }
            Err(err) => {
                logging::log!("Error adding project-area relation: {}", err);
                self.is_loading.1.try_update(|v| *v = false);
                Err(err)
            }
        }
    }

    pub async fn remove_project_relations(&self, project_id: i64) -> Result<(), String> {
        self.is_loading.1.try_update(|v| *v = true);
        
        let url = format!("/rest/v1/catalog?project_id=eq.{}", project_id);
        match supabase_delete(&url).await {
            Ok(_) => {
                // Remove from local state
                self.catalog.1.update(|catalog| {
                    catalog.retain(|c| c.project_id != project_id);
                });
                self.is_loading.1.try_update(|v| *v = false);
                Ok(())
            }
            Err(err) => {
                logging::log!("Error removing project relations: {}", err);
                self.is_loading.1.try_update(|v| *v = false);
                Err(err)
            }
        }
    }

    pub async fn sync_project_areas(&self, project_id: i64, area_ids: std::collections::HashSet<i64>) -> Result<(), String> {
        // First remove existing relations
        if let Err(e) = self.remove_project_relations(project_id).await {
            return Err(e);
        }
        
        // Then add new relations
        for area_id in area_ids {
            if let Err(e) = self.add_project_area_relation(project_id, area_id).await {
                return Err(e);
            }
        }
        
        Ok(())
    }

    pub fn get_project_areas(&self, project_id: i64) -> Vec<i64> {
        let current_catalog = self.catalog.0.get();
        current_catalog
            .into_iter()
            .filter(|c| c.project_id == project_id)
            .map(|c| c.area_id)
            .collect()
    }


}

#[component]
pub fn CatalogContextProvider(children: Children) -> impl IntoView {
    let editor_context = Arc::new(CatalogContext::new());
   
    provide_context(editor_context);

    children()
}

#[component]
pub fn CatalogRoute(children: Children) -> impl IntoView {
    let catalog_context = use_catalog();
    spawn_local(async move {
        catalog_context.fetch_catalog().await;
    });

    children()
}


pub fn use_catalog() -> Arc<CatalogContext> {
    use_context::<Arc<CatalogContext>>().expect(
        "CatalogContext not found. Make sure you are using CatalogContextProvider."
    )
}
