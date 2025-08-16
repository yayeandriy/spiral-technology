#![allow(unused)]


use std::rc::Rc;
use leptos::{component, ev, logging, prelude::*};
use leptos_use::{use_element_size, use_event_listener, use_mouse, use_throttle_fn, UseElementSizeReturn};
use web_sys::HtmlDivElement;


#[component]
pub fn SliderComponent(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(optional)] initial: Option<f64>,
    #[prop(optional)] on_change_async: Option<Rc<dyn Fn(f64) -> i32>>,
    #[prop(optional)] on_change: Option<Rc<dyn Fn(f64) -> i32>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] label: Option<String>,
) -> impl IntoView {
    let track_ref: NodeRef<leptos::html::Div> = NodeRef::new(); 
    let UseElementSizeReturn{width, height} = use_element_size(track_ref.clone());
    let mouse = use_mouse();

    let value = signal(initial.unwrap_or(min));
    let percent = move || {
        let value = value.0.get();
        if max - min == 0.0 {
            0.0
        } else {
            (value - min) / (max - min)
        }
    };

    let is_dragging = signal(false);

    // Effect::new(move |_| {
    //     let on_global_up = {
    //         let is_dragging = is_dragging.clone();
    //         move |_| {
    //             is_dragging.1.set(false);      
    //         }
    //     };
    //     logging::log!("SliderComponent: Adding global mouseup listener");
    //     use_event_listener(window(), ev::mouseup, on_global_up);
    // });

    let on_change_async_moved = on_change_async.clone();     
    let throttled = use_throttle_fn(
        move || {
            let v = value.0.get();
            if let Some(cb) = &on_change_async_moved {
                cb(v);
            }
        },
        300.0, // ms
    );

    let on_change_moved = on_change;
    Effect::new(move |_| {
       let is_dragging = signal(false);
        
       let on_down = {
            let is_dragging = is_dragging.clone();
            move |_| {
                is_dragging.1.set(true);      
            }
        };
       let on_up = {
            let is_dragging = is_dragging.clone();
            move |_| {
                is_dragging.1.set(false);      
            }
        };

        use_event_listener(track_ref, ev::mousedown, on_down);
        use_event_listener(track_ref, ev::mouseup, on_up);
        
        let on_change_moved = on_change_moved.clone();
        let throttled = throttled.clone();
        Effect::new( move |_|{
            if !is_dragging.0.get() {
                return;
            }
            let mouse_x = mouse.x.get() as f64;
            let track_x = track_ref.get().map_or::<f64, _>(0.0, |el: HtmlDivElement| el.get_bounding_client_rect().x() as f64);
            let width = width.get() as f64;

            if width > 0.0 {
                let ratio = ((mouse_x - track_x) / width).clamp(0.0, 1.0);
                let raw_val = min + ratio * (max - min);
                let stepped = ((raw_val - min) / step).round() * step + min;
                value.1.set(stepped.clamp(min, max));
                let stepped = ((raw_val - min) / step).round() * step + min;
                value.1.set(stepped.clamp(min, max));
                throttled(); // Call the throttled function
                if let Some(cb) = &on_change_moved {
                        cb(stepped);
                }
            }

        });
                
        
    });


    let label = match label {
        Some(label) => view! { <label class=" select-none ml-1  uppercase absolute font-mono text-[9px] tracking-wider text-slate-200">{label}</label> }.into_any(),
        None => view! {<div/>}.into_any(),
    };

    view! {
            <div
            node_ref=track_ref
            class=move || class.clone().unwrap_or("w-full opacity-50 border hover:opacity-100 transition-opacity h-4 border-slate-500 transition-none rounded relative cursor-pointer".into())
        >
            <div
                class="absolute top-0 bottom-0 bg-slate-500 rounded-l transition-none"
                style=move || format!("width: {:.2}%;", percent() * 100.0)
            />
            {label}
        </div>
    
    
    
    }

}
