use leptos::prelude::*;
use leptos::ev::Event;

#[component]
pub fn Checkbox(
    /// Checkbox checked state
    #[prop(into)]
    checked: Signal<bool>,
    /// Checkbox name
    #[prop(default = "".to_string())]
    name: String,
    /// Checkbox id
    #[prop(default = "".to_string())]
    id: String,
    /// Checkbox value
    #[prop(default = "".to_string())]
    value: String,
    /// Whether the checkbox is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(Event)>>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let base_classes = "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded";
    
    let classes = format!("{} {}", base_classes, class);

    view! {
        <input
            type="checkbox"
            class=classes
            prop:checked=move || checked.get()
            name=name
            id=id
            value=value
            disabled=disabled
            on:change=move |ev| {
                if let Some(ref handler) = on_change {
                    handler(ev);
                }
            }
        />
    }
}

#[component]
pub fn CheckboxWithLabel(
    /// Checkbox checked state
    #[prop(into)]
    checked: Signal<bool>,
    /// Label content
    children: Children,
    /// Checkbox name
    #[prop(default = "".to_string())]
    name: String,
    /// Checkbox id (will be generated if not provided)
    #[prop(optional)]
    id: Option<String>,
    /// Checkbox value
    #[prop(default = "".to_string())]
    value: String,
    /// Whether the checkbox is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(Event)>>,
    /// Additional CSS classes for checkbox
    #[prop(default = "".to_string())]
    class: String,
    /// Additional CSS classes for container
    #[prop(default = "".to_string())]
    container_class: String,
) -> impl IntoView {
    let checkbox_id = id.unwrap_or_else(|| format!("checkbox_{}_{}", name, value));
    
    view! {
        <div class={format!("flex items-start space-x-3 {}", container_class)}>
            {if let Some(handler) = on_change {
                view! {
                    <Checkbox
                        checked=checked
                        name=name
                        id=checkbox_id.clone()
                        value=value
                        disabled=disabled
                        on_change=handler
                        class=class
                    />
                }
            } else {
                view! {
                    <Checkbox
                        checked=checked
                        name=name
                        id=checkbox_id.clone()
                        value=value
                        disabled=disabled
                        class=class
                    />
                }
            }}
            <label 
                for=checkbox_id
                class="flex-1 cursor-pointer"
            >
                {children()}
            </label>
        </div>
    }
}

// Area selector checkbox (specific to the app's area selection functionality)
#[component]
pub fn AreaCheckbox(
    /// Area ID
    area_id: i64,
    /// Area title
    title: String,
    /// Area description (optional)
    description: Option<String>,
    /// Whether this area is selected
    #[prop(into)]
    selected: Signal<bool>,
    /// Whether the checkbox is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(Event)>>,
) -> impl IntoView {
    let checkbox_id = format!("area_{}", area_id);
    
    view! {
        {if let Some(handler) = on_change {
            view! {
                <CheckboxWithLabel
                    checked=selected
                    name="".to_string()
                    id=checkbox_id.clone()
                    value=area_id.to_string()
                    disabled=disabled
                    on_change=handler
                    class="mt-1".to_string()
                >
                    <div>
                        <div class="text-sm font-medium text-gray-900">
                            {title}
                        </div>
                        {description.map(|desc| view! {
                            <div class="text-xs text-gray-500 mt-1">
                                {desc}
                            </div>
                        })}
                    </div>
                </CheckboxWithLabel>
            }
        } else {
            view! {
                <CheckboxWithLabel
                    checked=selected
                    name="".to_string()
                    id=checkbox_id.clone()
                    value=area_id.to_string()
                    disabled=disabled
                    class="mt-1".to_string()
                >
                    <div>
                        <div class="text-sm font-medium text-gray-900">
                            {title}
                        </div>
                        {description.map(|desc| view! {
                            <div class="text-xs text-gray-500 mt-1">
                                {desc}
                            </div>
                        })}
                    </div>
                </CheckboxWithLabel>
            }
        }}
    }
}
