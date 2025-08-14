use leptos::prelude::*;

#[component]
pub fn Label(
    /// Label content
    children: Children,
    /// For attribute (links to input id)
    #[prop(default = "".to_string())]
    for_: String,
    /// Whether this label is for a required field
    #[prop(default = false)]
    required: bool,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let base_classes = "block text-sm font-medium text-gray-700";
    let classes = format!("{} {}", base_classes, class);

    view! {
        <label 
            for=for_
            class=classes
        >
            {children()}
            {required.then(|| view! {
                <span class="text-red-500 ml-1">*</span>
            })}
        </label>
    }
}

#[component]
pub fn FormLabel(
    /// Label text
    text: String,
    /// For attribute (links to input id)
    #[prop(default = "".to_string())]
    for_: String,
    /// Whether this label is for a required field
    #[prop(default = false)]
    required: bool,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    view! {
        <Label
            for_=for_
            required=required
            class=class
        >
            {text}
        </Label>
    }
}

#[component] 
pub fn FieldLabel(
    /// Label text
    text: String,
    /// For attribute (links to input id) 
    #[prop(default = "".to_string())]
    for_: String,
    /// Whether this label is for a required field
    #[prop(default = false)]
    required: bool,
    /// Additional CSS classes
    #[prop(default = "mb-1".to_string())]
    class: String,
) -> impl IntoView {
    view! {
        <FormLabel
            text=text
            for_=for_
            required=required
            class=class
        />
    }
}
