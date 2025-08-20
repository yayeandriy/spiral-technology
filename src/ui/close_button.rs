use leptos::prelude::*;

#[component]
pub fn CloseButton(
    /// The URL to navigate to when clicked
    #[prop(default = "/home".to_string())]
    href: String,
    /// Additional CSS classes to apply
    #[prop(default = "".to_string())]
    class: String,
    /// Click handler (optional, if provided, href navigation is disabled)
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let base_classes = "w-12 h-12 text-lg rounded-full bg-gray-100 flex items-center justify-center cursor-pointer hover:bg-gray-200 transition-colors duration-200";
    let combined_classes = if class.is_empty() {
        base_classes.to_string()
    } else {
        format!("{} {}", base_classes, class)
    };

    view! {
        {
            if let Some(click_handler) = on_click {
                view! {
                    <button 
                        class=combined_classes
                        on:click=move |_| click_handler()
                    >
                        "╳"
                    </button>
                }.into_any()
            } else {
                view! {
                    <a 
                        href=href
                        class=combined_classes
                    >
                        "╳"
                    </a>
                }.into_any()
            }
        }
    }
}
