use std::{collections::HashMap, sync::Arc};

use leptos::prelude::{signal, ReadSignal, WriteSignal};

// Re-export the MarkdownHandler trait for convenience
pub use crate::ui::text_editor::markdown_trait::MarkdownHandler;


#[derive(Clone)]
pub struct DataState<T = (), P = ()> 
where 
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
{
    pub data: HashMap<String, (ReadSignal<String>, WriteSignal<String>)>,
    pub is_modified: (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>),
    pub id: i32,
    pub created_at: String,
    pub init_data: Option<T>,
    pub context: Option<Arc<P>>,
}

impl<T, P> Default for DataState<T,P> 
where 
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            is_modified: signal(vec![]),
            id: 0,
            created_at: String::new(),
            init_data: None,
            context: None,
        }
    }

    

}

pub trait DataHandler {
    fn update_or_create(&self);
}

impl<T, P> DataState<T, P> 
where 
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
{
    // The update_or_create method should be implemented in specific modules
    // for their particular DataState types
}

