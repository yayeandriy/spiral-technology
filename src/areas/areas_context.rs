use leptos::{
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

use crate::{areas::model::ProjectArea, supabase::supabase_get};



#[derive(Clone)]
pub struct AreaContext {
    pub areas: (ReadSignal<Vec<ProjectArea>>, WriteSignal<Vec<ProjectArea>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    url_path: String,
}

impl AreaContext {
    pub fn new() -> Self {
        Self {
            areas: signal::<Vec<ProjectArea>>(vec![]),
            is_loading: signal(false),
            error: signal(None),
            url_path: format!("/rest/v1/areas?select=*&host=eq.roboscope"),
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
                logging::log!("Fetched areas: {:?}", items);
                self.areas.1.set(items);
            }
            Err(err) => {
                logging::log!("Error fetching items: {}", err);
            }
        }
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
