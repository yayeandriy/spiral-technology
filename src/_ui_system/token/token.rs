#![allow(unused)]


use std::rc::Rc;
use leptos::{children, component, ev, logging, prelude::*};
use leptos_use::{use_element_size, use_event_listener, use_mouse, use_throttle_fn, UseElementSizeReturn};
use web_sys::{HtmlDivElement, MouseEvent};

use crate::shared::ui_components::icon::icon::Icon;

#[derive(Clone, Debug)]
pub enum TokenSize {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Debug)]
pub enum TokenStyle {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Ghost
}

#[component]
pub fn Token(
    value: Option<String>,
    #[prop(into)]
    #[prop(optional)]
    size: Option<TokenSize>,
    #[prop(into)]
    #[prop(optional)]
    style: Option<TokenStyle>,
    #[prop(into)]
    #[prop(optional)]
    icon: Option<String>,
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = Some(false))] 
    full: Option<bool>
) -> impl IntoView {

    let res = if let Some(full) = full {
        if full {
            value.clone()
        } else {
            value.map(|id| id.chars().take(10).collect::<String>())
        }
    } else {
        value.map(|id| id.chars().take(10).collect::<String>())
    };
    let scale = match size {
        Some(TokenSize::Small) => "text-[9px] px-[3px] py-[1px]",
        Some(TokenSize::Medium) => "text-[10px] px-[4px] py-[2px]",
        Some(TokenSize::Large) => "text-[11px] px-[5px] py-[3px]",
        Some(TokenSize::XLarge) => "text-[12px] px-[8px] py-[5px]",
        None => "text-[9px] px-[3px] py-[1px]",
    };
    let style_class = match style {
        Some(TokenStyle::Primary) => "bg-blue-500 text-white",
        Some(TokenStyle::Secondary) => "bg-gray-500 text-white",
        Some(TokenStyle::Success) => "bg-green-500 text-white",
        Some(TokenStyle::Danger) => "bg-red-500 text-white",
        Some(TokenStyle::Warning) => "bg-yellow-500 text-black",
        Some(TokenStyle::Info) => "bg-cyan-500 text-white",
        Some(TokenStyle::Ghost) => "bg-transparent text-cyan-300",
        None => "bg-cyan-900 text-cyan-500",
    };

    view! {
        {
            if res.is_none() {
                return view! { <div></div> }.into_any();
            }else{
                view! {

                    <span class={format!("flex items-center gap-1 rounded font-mono uppercase tracking-wider {} {}", scale, style_class)}
                    title="ID Tag"
                    >
                        {
                            if let Some(icon) = icon {
                                view! {
                                    <Icon name={icon} size=size.clone().unwrap_or(TokenSize::Small) />
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }
                        }
                        <span>
                            {res} 
                        </span>
                    </span>
                }.into_any()
            }
        }
        
    }
}
