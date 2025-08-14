use leptos::prelude::*;
use leptos::ev::Event;

use super::input::InputState;

#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    
    pub fn new_disabled(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: true,
        }
    }
}

#[component]
pub fn Select(
    /// Select value
    #[prop(into)]
    value: Signal<String>,
    /// Select options
    options: Vec<SelectOption>,
    /// Select name
    #[prop(default = "".to_string())]
    name: String,
    /// Select id
    #[prop(default = "".to_string())]
    id: String,
    /// Whether the select is required
    #[prop(default = false)]
    required: bool,
    /// Whether the select is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Select state for styling
    #[prop(default = InputState::Normal)]
    state: InputState,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(Event)>>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
    /// Placeholder option text
    #[prop(optional)]
    placeholder: Option<String>,
) -> impl IntoView {
    let base_classes = "w-full text-black px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-offset-0 appearance-none bg-white";
    
    let classes = format!(
        "{} {} {}",
        base_classes,
        state.classes(),
        class
    );

    view! {
        <div class="relative">
            <select
                class=classes
                prop:value=move || value.get()
                name=name
                id=id
                required=required
                disabled=disabled
                on:change=move |ev| {
                    if let Some(ref handler) = on_change {
                        handler(ev);
                    }
                }
            >
                {placeholder.map(|placeholder_text| view! {
                    <option value="">
                        {placeholder_text}
                    </option>
                })}
                {options.into_iter().map(|option| {
                    let value = option.value.clone();
                    view! {
                        <option value=value disabled=option.disabled>
                            {option.label}
                        </option>
                    }
                }).collect::<Vec<_>>()}
            </select>
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                    <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/>
                </svg>
            </div>
        </div>
    }
}

// Component for category selector (used in catalog editor)
#[component]
pub fn CategorySelect(
    /// Select value
    #[prop(into)]
    value: Signal<String>,
    /// Available categories
    categories: Vec<String>,
    /// Select name
    #[prop(default = "category".to_string())]
    name: String,
    /// Select id
    #[prop(default = "".to_string())]
    id: String,
    /// Whether the select is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Select state for styling
    #[prop(default = InputState::Normal)]
    state: InputState,
    /// Change event handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(Event)>>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
    /// Whether to show "Add new category" option
    #[prop(default = true)]
    allow_custom: bool,
) -> impl IntoView {
    let mut options = categories.into_iter()
        .map(|cat| SelectOption::new(cat.clone(), cat))
        .collect::<Vec<_>>();
        
    if allow_custom {
        options.push(SelectOption::new("__custom__", "Add new category..."));
    }

    view! {
        {if let Some(handler) = on_change {
            view! {
                <Select
                    value=value
                    options=options
                    name=name
                    id=id
                    disabled=disabled
                    state=state
                    on_change=handler
                    class=class
                    placeholder="Select a category...".to_string()
                />
            }
        } else {
            view! {
                <Select
                    value=value
                    options=options
                    name=name
                    id=id
                    disabled=disabled
                    state=state
                    class=class
                    placeholder="Select a category...".to_string()
                />
            }
        }}
    }
}
