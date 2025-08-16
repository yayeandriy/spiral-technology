


use leptos::html::Button;

use leptos::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn};
use web_sys::MouseEvent;

use crate::shared::ui_components::token::token::{TokenSize, TokenStyle};


#[component]
pub fn ButtonContainer(
    on_click: impl FnMut(MouseEvent) + 'static,
    children: Children,

    #[prop(into)]
    #[prop(optional)]
    size: Option<TokenSize>,
    #[prop(into)]
    #[prop(optional)]
    style: Option<TokenStyle>,
) -> impl IntoView {
    let size = match size {
        Some(TokenSize::Small) => "text-[9px] mb-1",
        Some(TokenSize::Medium) => "text-[10px] mb-2",
        Some(TokenSize::Large) => "text-[11px] mb-3",
        Some(TokenSize::XLarge) => "text-[11px] mb-3",
        _ => "text-[10px] px-2 h-8",
    };
    let style_class = match style {
        Some(TokenStyle::Primary) => "bg-teal-500 hover:bg-teal-600 text-white",
        Some(TokenStyle::Secondary) => "bg-slate-500 hover:bg-slate-600 text-slate-200",
        Some(TokenStyle::Success) => "text-white",
        Some(TokenStyle::Danger) => "text-white",
        Some(TokenStyle::Warning) => "text-black",
        Some(TokenStyle::Info) => "text-white",
        _ => "border text-white border-white   ",
    };


    
    
    view! {
        <button on:click=on_click class={format!("flex semibold uppercase tracking-wider items-center justify-center rounded w-full {} {}", size, style_class)} >
            <span class="truncate " >
                {children()}
             </span>
        </button>
    }
}
