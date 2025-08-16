#![allow(unused)]


use std::rc::Rc;
use leptos::{children, component, ev, logging, prelude::*};
use leptos_use::{use_element_size, use_event_listener, use_mouse, use_throttle_fn, UseElementSizeReturn};
use web_sys::{HtmlDivElement, MouseEvent};


#[component]
pub fn Tag(
    on_select: impl FnMut(MouseEvent) + 'static,
    on_delete: impl FnMut(MouseEvent) + 'static,
    selected: impl Fn() -> bool + Send + 'static,
    children: Children,
) -> impl IntoView {
    let content = view!{
        <div
            on:click=on_select
             class="group-hover:text-cyan-800 p-1">
                {children()}
                
            </div>
            <div 
            on:click=on_delete
            class="p-[2px] mr-1 group-hover:bg-slate-500 group-hover:text-slate-200 rounded-full w-[15px] h-[15px] flex items-center justify-center ">
                <div class="h-[15px]" style="line-height: 1;" >
                    "×"
                </div>
            </div>
    };

    let class = move || {
        if selected() {
            "bg-blue-500 border border-transparent group hover:bg-blue-400 text-blue-200 rounded cursor-pointer flex items-center gap-1"
        } else {
            "bg-transparent border border-blue-500 group hover:border-blue-400 text-blue-400 hover:text-blue-500 rounded cursor-pointer flex items-center gap-1"
        }
    };

    view! {
        <div class=move || class()>
            {content}
        </div>
    }
}
