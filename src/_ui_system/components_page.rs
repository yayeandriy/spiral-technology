use leptos::{logging, prelude::*};
use std::rc::Rc;

use crate::shared::ui_components::slider::slider_component::SliderComponent;


 #[component]
pub fn ComponentsPage() -> impl IntoView {

    let handle_slider_change = Rc::new(move |value: f64| -> i32 {
        logging::log!("Slider value changed to: {}", value);
        0 // Return an i32 value
    }); 

    let handle_slider_change_async = Rc::new(move |value: f64| -> i32 {
        logging::log!("Slider value changed asynchronously to: {}", value);
        0 // Return an i32 value
    });

    view! {
        <main class="w-2/3 h-screen flex bg-gray-600 items-start justify-start p-8">
           <SliderComponent
            min=0.0
            on_change=handle_slider_change
            on_change_async=handle_slider_change_async
            // max=10.0
            // step=0.1
            initial=5.0
            // class="h-6 bg-gray-100 rounded"
        /> 
        </main>
    }
}
