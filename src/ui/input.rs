use leptos::prelude::*;
use leptos::ev::Event;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputState {
    Normal,
    Error,
    Disabled,
}

impl InputState {
    pub fn classes(&self) -> &'static str {
        match self {
            InputState::Normal => "border-gray-300 focus:border-blue-500 focus:ring-blue-500",
            InputState::Error => "border-red-300 focus:border-red-500 focus:ring-red-500",
            InputState::Disabled => "border-gray-300 bg-gray-50 text-gray-500",
        }
    }
}

#[component]
pub fn Input(
    /// Input value
    #[prop(into)]
    value: Signal<String>,
    /// Input placeholder
    #[prop(default = "".to_string())]
    placeholder: String,
    /// Input type (text, email, password, etc.)
    #[prop(default = "text".to_string())]
    type_: String,
    /// Input name
    #[prop(default = "".to_string())]
    name: String,
    /// Input id
    #[prop(default = "".to_string())]
    id: String,
    /// Whether the input is required
    #[prop(default = false)]
    required: bool,
    /// Whether the input is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Input state for styling
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
    node_ref: Option<NodeRef<leptos::html::Input>>,
) -> impl IntoView {
    let base_classes = "w-full px-3 py-2 text-black border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-0";
    
    let classes = format!(
        "{} {} {}",
        base_classes,
        state.classes(),
        class
    );

    view! {
        <input
            type=type_
            class=classes
            placeholder=placeholder
            prop:value=move || value.get()
            name=name
            id=id
            required=required
            disabled=disabled
            on:input=move |ev| {
                if let Some(ref handler) = on_input {
                    handler(ev);
                }
            }
            node_ref=node_ref.unwrap_or_default()
        />
    }
}

#[component]
pub fn TextInput(
    /// Input value
    #[prop(into)]
    value: Signal<String>,
    /// Input placeholder
    #[prop(default = "".to_string())]
    placeholder: String,
    /// Input name
    #[prop(default = "".to_string())]
    name: String,
    /// Input id
    #[prop(default = "".to_string())]
    id: String,
    /// Whether the input is required
    #[prop(default = false)]
    required: bool,
    /// Whether the input is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Input state for styling
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
    node_ref: Option<NodeRef<leptos::html::Input>>,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_input {
            if let Some(node_ref) = node_ref {
                view! {
                    <Input
                        value=value
                        type_="text".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
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
                    <Input
                        value=value
                        type_="text".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
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
                    <Input
                        value=value
                        type_="text".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
                        required=required
                        disabled=disabled
                        state=state
                        class=class
                        node_ref=node_ref
                    />
                }
            } else {
                view! {
                    <Input
                        value=value
                        type_="text".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
                        required=required
                        disabled=disabled
                        state=state
                        class=class
                    />
                }
            }
        }}
    }
}

#[component]
pub fn EmailInput(
    /// Input value
    #[prop(into)]
    value: Signal<String>,
    /// Input placeholder
    #[prop(default = "Enter email address...".to_string())]
    placeholder: String,
    /// Input name
    #[prop(default = "email".to_string())]
    name: String,
    /// Input id
    #[prop(default = "".to_string())]
    id: String,
    /// Whether the input is required
    #[prop(default = false)]
    required: bool,
    /// Whether the input is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Input state for styling
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
    node_ref: Option<NodeRef<leptos::html::Input>>,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_input {
            if let Some(node_ref) = node_ref {
                view! {
                    <Input
                        value=value
                        type_="email".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
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
                    <Input
                        value=value
                        type_="email".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
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
                    <Input
                        value=value
                        type_="email".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
                        required=required
                        disabled=disabled
                        state=state
                        class=class
                        node_ref=node_ref
                    />
                }
            } else {
                view! {
                    <Input
                        value=value
                        type_="email".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
                        required=required
                        disabled=disabled
                        state=state
                        class=class
                    />
                }
            }
        }}
    }
}

#[component]
pub fn PasswordInput(
    /// Input value
    #[prop(into)]
    value: Signal<String>,
    /// Input placeholder
    #[prop(default = "Enter password...".to_string())]
    placeholder: String,
    /// Input name
    #[prop(default = "password".to_string())]
    name: String,
    /// Input id
    #[prop(default = "".to_string())]
    id: String,
    /// Whether the input is required
    #[prop(default = false)]
    required: bool,
    /// Whether the input is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Input state for styling
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
    node_ref: Option<NodeRef<leptos::html::Input>>,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_input {
            if let Some(node_ref) = node_ref {
                view! {
                    <Input
                        value=value
                        type_="password".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
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
                    <Input
                        value=value
                        type_="password".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
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
                    <Input
                        value=value
                        type_="password".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
                        required=required
                        disabled=disabled
                        state=state
                        class=class
                        node_ref=node_ref
                    />
                }
            } else {
                view! {
                    <Input
                        value=value
                        type_="password".to_string()
                        placeholder=placeholder
                        name=name
                        id=id
                        required=required
                        disabled=disabled
                        state=state
                        class=class
                    />
                }
            }
        }}
    }
}
