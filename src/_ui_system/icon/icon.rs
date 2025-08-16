#![allow(unused)]


use std::rc::Rc;
use leptos::{children, component, ev, logging, prelude::*};
use leptos_use::{use_element_size, use_event_listener, use_mouse, use_throttle_fn, UseElementSizeReturn};
use web_sys::{HtmlDivElement, MouseEvent};

use crate::shared::ui_components::token::token::TokenSize;



#[component]
pub fn Icon(
    #[prop(into)]
    #[prop(optional)]
    name: Option<String>,    
    #[prop(into)]
    #[prop(optional)]
    size: Option<TokenSize>,
) -> impl IntoView {
    let name = name.unwrap_or_else(|| "default_icon".to_string());

    let scale = match size {
        Some(TokenSize::Small) => "w-4 h-4",
        Some(TokenSize::Medium) => "w-4 h-4",
        Some(TokenSize::Large) => "w-4 h-4",
        Some(TokenSize::XLarge) => "w-4 h-4",
        None => "w-4 h-4",
    };
    view! {
            <img 
                src=format!("/public/icons/{}.svg", name)
                alt=name
                class=format!("{} object-contain", scale)
            />
        }
     
}
