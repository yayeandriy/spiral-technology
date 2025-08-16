use leptos::prelude::*;



#[component]
pub fn Img(
    src: String,
    w: u32,    
) -> impl IntoView {
    let (loaded, set_loaded) = signal(false);
    let (error, set_error) = signal(false);
   
    let container_style = move || {
        let mut style = format!("width: {}px;", w);
        style.push_str(" aspect-ratio: 3/4;");
        style.push_str(" position: relative; overflow: hidden; display: flex; align-items: center; justify-content: center;");
        style
    };

    let loaded_clone = loaded.clone();
    let img_class = move || {
        let mut classes = "w-full h-full object-cover transition-opacity duration-300".to_string();
        if !loaded_clone.get() {
            classes.push_str(" opacity-0");
        } else {
            classes.push_str(" opacity-100");
        }

        classes
    };
    let loaded_clone = loaded.clone();
    let error_clone = error.clone();
    let set_loaded_clone = set_loaded.clone();
    let set_error_clone = set_error.clone();
    view! {
        <div style={container_style} class="bg-slate-600">
            {move || {
                if !loaded_clone.get() && !error_clone.get() {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-400"></div>
                        </div>
                    }.into_any()
                } else if error.get() {
                    view! {
                        <div class="absolute inset-0 flex items-center justify-center text-gray-400">
                            <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clip-rule="evenodd"/>
                            </svg>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
            <img 
                src={src} 
                alt="Turbine defect"
                class={img_class}
                on:load=move |_| set_loaded_clone.set(true)
                on:error=move |_| set_error_clone.set(true)
            />
        </div>
    }
}
