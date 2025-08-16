#![allow(unused)]


use std::rc::Rc;
use leptos::{children, component, ev, logging, prelude::*};
use leptos_use::{use_element_size, use_event_listener, use_mouse, use_throttle_fn, UseElementSizeReturn};
use web_sys::{HtmlDivElement, MouseEvent};

use crate::shared::ui_components::token::token::{TokenSize, TokenStyle};




#[component]
pub fn Close(
    
) -> impl IntoView {
   

    view! {
            
             <div                
                class="text-slate-100 m-1 items-center justify-center w-5 h-5 flex flex-col cursor-pointer rounded-full bg-slate-500  hover:bg-slate-400 transition-colors duration-200"
            >
            <span class="">"✖"</span></div>
        
    }
}
