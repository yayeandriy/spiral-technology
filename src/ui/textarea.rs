use leptos::prelude::*;
use leptos::ev::Event;

use super::input::InputState;

#[component]
pub fn TextArea(
    /// TextArea value
    #[prop(into)]
    value: Signal<String>,
    /// TextArea placeholder
    #[prop(default = "".to_string())]
    placeholder: String,
    /// TextArea name
    #[prop(default = "".to_string())]
    name: String,
    /// TextArea id
    #[prop(default = "".to_string())]
    id: String,
    /// Number of rows
    #[prop(default = 3)]
    rows: u32,
    /// Maximum number of characters
    #[prop(optional)]
    max_length: Option<u32>,
    /// Whether the textarea is required
    #[prop(default = false)]
    required: bool,
    /// Whether the textarea is disabled
    #[prop(default = false)]
    disabled: bool,
    /// TextArea state for styling
    #[prop(default = InputState::Normal)]
    state: InputState,
    /// Input event handler
    #[prop(optional)]
    on_input: Option<Box<dyn Fn(Event)>>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Textarea>>,
) -> impl IntoView {
    let base_classes = "w-full px-3 py-2 text-black border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-0 resize-vertical";
    
    let classes = format!(
        "{} {} {}",
        base_classes,
        state.classes(),
        class
    );

    view! {
        <textarea
            class=classes
            placeholder=placeholder
            prop:value=move || value.get()
            name=name
            id=id
            rows=rows
            maxlength=max_length.map(|len| len.to_string()).unwrap_or_default()
            required=required
            disabled=disabled
            on:input=move |ev| {
                if let Some(ref handler) = on_input {
                    handler(ev);
                }
            }
            node_ref=node_ref.unwrap_or_default()
        >
        </textarea>
    }
}

// Character counter component that can be used with TextArea
#[component]
pub fn CharacterCounter(
    /// Current character count
    current: Signal<usize>,
    /// Maximum character count
    max: u32,
) -> impl IntoView {
    let max_usize = max as usize;
    
    view! {
        <p class="text-xs text-gray-500">
            {move || {
                let count = current.get();
                let color_class = if count > max_usize {
                    "text-red-500"
                } else if count > (max_usize * 90 / 100) {
                    "text-yellow-500"
                } else {
                    "text-gray-500"
                };
                format!("{}/{} characters", count, max)
            }}
        </p>
    }
}

// TextArea with built-in character counter
#[component]
pub fn TextAreaWithCounter(
    /// TextArea value
    #[prop(into)]
    value: Signal<String>,
    /// Maximum number of characters
    max_length: u32,
    /// TextArea placeholder
    #[prop(default = "".to_string())]
    placeholder: String,
    /// TextArea name
    #[prop(default = "".to_string())]
    name: String,
    /// TextArea id
    #[prop(default = "".to_string())]
    id: String,
    /// Number of rows
    #[prop(default = 3)]
    rows: u32,
    /// Whether the textarea is required
    #[prop(default = false)]
    required: bool,
    /// Whether the textarea is disabled
    #[prop(default = false)]
    disabled: bool,
    /// TextArea state for styling
    #[prop(default = InputState::Normal)]
    state: InputState,
    /// Input event handler
    #[prop(optional)]
    on_input: Option<Box<dyn Fn(Event)>>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Textarea>>,
) -> impl IntoView {
    let char_count = Signal::derive(move || value.get().len());
    
    view! {
        <div class="space-y-1">
            {if let Some(handler) = on_input {
                if let Some(node_ref) = node_ref {
                    view! {
                        <TextArea
                            value=value
                            placeholder=placeholder
                            name=name
                            id=id
                            rows=rows
                            max_length=max_length
                            required=required
                            disabled=disabled
                            state=state
                            on_input=handler
                            class=class
                            node_ref=node_ref
                        />
                    }
                } else {
                    view! {
                        <TextArea
                            value=value
                            placeholder=placeholder
                            name=name
                            id=id
                            rows=rows
                            max_length=max_length
                            required=required
                            disabled=disabled
                            state=state
                            on_input=handler
                            class=class
                        />
                    }
                }
            } else {
                if let Some(node_ref) = node_ref {
                    view! {
                        <TextArea
                            value=value
                            placeholder=placeholder
                            name=name
                            id=id
                            rows=rows
                            max_length=max_length
                            required=required
                            disabled=disabled
                            state=state
                            class=class
                            node_ref=node_ref
                        />
                    }
                } else {
                    view! {
                        <TextArea
                            value=value
                            placeholder=placeholder
                            name=name
                            id=id
                            rows=rows
                            max_length=max_length
                            required=required
                            disabled=disabled
                            state=state
                            class=class
                        />
                    }
                }
            }}
            <CharacterCounter
                current=char_count
                max=max_length
            />
        </div>
    }
}
