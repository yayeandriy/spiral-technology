#![allow(unused)]


use std::rc::Rc;
use leptos::{children, component, ev, logging, prelude::*};
use leptos_use::{use_element_size, use_event_listener, use_mouse, use_throttle_fn, UseElementSizeReturn};
use web_sys::{HtmlDivElement, MouseEvent};

use crate::shared::ui_components::token::token::{TokenSize, TokenStyle};




#[component]
pub fn Caption(
    children: Children,
    #[prop(into)]
    #[prop(optional)]
    size: Option<TokenSize>,
    #[prop(into)]
    #[prop(optional)]
    style: Option<TokenStyle>,
) -> impl IntoView {
    let scale = match size {
        Some(TokenSize::Small) => "text-[9px] mb-1",
        Some(TokenSize::Medium) => "text-[10px] mb-2",
        Some(TokenSize::Large) => "text-[11px] mb-3",
        Some(TokenSize::XLarge) => "text-[11px] mb-3",
        _ => "text-[9px]",
    };
    let style_class = match style {
        Some(TokenStyle::Primary) => "text-white",
        Some(TokenStyle::Secondary) => "text-white",
        Some(TokenStyle::Success) => "text-white",
        Some(TokenStyle::Danger) => "text-white",
        Some(TokenStyle::Warning) => "text-black",
        Some(TokenStyle::Info) => "text-white",
        _ => "text-cyan-500",
    };

    view! {
            
            <span class={format!("uppercase tracking-wider {} {}", scale, style_class)}
            title="Caption"
            >
            {children()}            
            </span>
        
    }
}
