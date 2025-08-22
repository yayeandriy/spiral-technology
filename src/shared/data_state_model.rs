use std::{collections::HashMap, sync::Arc};

use leptos::prelude::{signal, ReadSignal, WriteSignal};


#[derive(Clone)]
pub struct DataState<T = (), P = ()> {
    pub data: HashMap<String, (ReadSignal<String>, WriteSignal<String>)>,
    pub is_modified: (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>),
    pub id: i32,
    pub created_at: String,
    pub init_data: Option<T>,
    pub context: Option<Arc<P>>,
}

impl<T, P> Default for DataState<T,P> {
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