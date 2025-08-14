use leptos::prelude::*;

#[component]
pub fn ErrorMessage(
    /// Error message text
    message: String,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let classes = format!("text-sm text-red-600 {}", class);
    
    view! {
        <p class=classes>
            {message}
        </p>
    }
}

#[component]
pub fn FieldError(
    /// Error message (optional)
    #[prop(optional)]
    error: Option<String>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    match error {
        Some(message) => view! {
            <ErrorMessage message=message class=class />
        }.into_any(),
        None => view! { <div></div> }.into_any(),
    }
}

#[component]
pub fn ValidationErrors(
    /// List of validation errors
    errors: Vec<String>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    if errors.is_empty() {
        return view! { <div></div> }.into_any();
    }

    view! {
        <div class={format!("p-4 bg-red-50 border border-red-200 rounded-md {}", class)}>
            <div class="flex">
                <div class="ml-3">
                    <h3 class="text-sm font-medium text-red-800">
                        "Please fix the following errors:"
                    </h3>
                    <ul class="mt-2 text-sm text-red-700 list-disc list-inside">
                        {errors.into_iter().map(|error| view! {
                            <li>{error}</li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn InfoMessage(
    /// Info message text
    message: String,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let classes = format!("text-xs text-gray-500 {}", class);
    
    view! {
        <p class=classes>
            {message}
        </p>
    }
}

#[component]
pub fn SuccessMessage(
    /// Success message text
    message: String,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let classes = format!("text-sm text-green-600 {}", class);
    
    view! {
        <p class=classes>
            {message}
        </p>
    }
}
