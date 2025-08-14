use crate::catalog::model::{ProjectCatalog};
use crate::supabase::supabase_get;
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
pub struct CatalogContext {
    pub catalog: (ReadSignal<Vec<ProjectCatalog>>, WriteSignal<Vec<ProjectCatalog>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    url_path: String,
}

impl CatalogContext {
    pub fn new() -> Self {
        Self {
            catalog: signal::<Vec<ProjectCatalog>>(vec![]),
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
        match supabase_get::<Vec<ProjectCatalog>>(&self.url_path).await {
            Ok(items) => {
                logging::log!("Fetched catalog successfully: {:?}", items);
                self.catalog.1.set(items);
            }
            Err(err) => {
                logging::log!("Error fetching catalog: {}", err);
            }
        }
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
