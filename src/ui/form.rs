use leptos::prelude::*;
use leptos::ev::Event;

use super::*;

#[component]
pub fn FormField(
    /// Form field content (input, textarea, select, etc.)
    children: Children,
    /// Label text
    label: String,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Optional error message
    #[prop(optional)]
    error: Option<String>,
    /// Optional help text
    #[prop(optional)]
    help: Option<String>,
    /// Additional CSS classes for the container
    #[prop(default = "space-y-2".to_string())]
    class: String,
    /// HTML id for the form field (will link label to input)
    #[prop(optional)]
    id: Option<String>,
) -> impl IntoView {
    let field_id = id.unwrap_or_else(|| format!("field_{}", label.to_lowercase().replace(" ", "_")));
    
    view! {
        <div class=class>
            <FieldLabel
                text=label
                for_=field_id.clone()
                required=required
            />
            {children()}
            {error.map(|err| view! {
                <ErrorMessage message=err />
            })}
            {help.map(|help_text| view! {
                <InfoMessage message=help_text />
            })}
        </div>
    }
}

#[component]
pub fn TextFormField(
    /// Input value
    #[prop(into)]
    value: Signal<String>,
    /// Label text
    label: String,
    /// Input placeholder
    #[prop(default = "".to_string())]
    placeholder: String,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Whether the input is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Optional error message
    #[prop(optional)]
    error: Option<String>,
    /// Optional help text
    #[prop(optional)]
    help: Option<String>,
    /// Input event handler
    #[prop(optional)]
    on_input: Option<Box<dyn Fn(Event) + Send + Sync>>,
    /// HTML name attribute
    #[prop(default = "".to_string())]
    name: String,
    /// Additional CSS classes for the container
    #[prop(default = "space-y-2".to_string())]
    class: String,
    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Input>>,
) -> impl IntoView {
    let field_id = if name.is_empty() {
        format!("field_{}", label.to_lowercase().replace(" ", "_"))
    } else {
        name.clone()
    };
    
    let state = if error.is_some() { InputState::Error } else { InputState::Normal };
    
    view! {
        <FormField
            label=label
            required=required
            error=error.unwrap_or_default()
            help=help.unwrap_or_default()
            class=class
            id=field_id.clone()
        >
            {if let Some(handler) = on_input {
                if let Some(node_ref) = node_ref {
                    view! {
                        <TextInput
                            value=value
                            placeholder=placeholder
                            name=name
                            id=field_id
                            required=required
                            disabled=disabled
                            state=state
                            on_input=handler
                            node_ref=node_ref
                        />
                    }
                } else {
                    view! {
                        <TextInput
                            value=value
                            placeholder=placeholder
                            name=name
                            id=field_id
                            required=required
                            disabled=disabled
                            state=state
                            on_input=handler
                        />
                    }
                }
            } else {
                if let Some(node_ref) = node_ref {
                    view! {
                        <TextInput
                            value=value
                            placeholder=placeholder
                            name=name
                            id=field_id
                            required=required
                            disabled=disabled
                            state=state
                            node_ref=node_ref
                        />
                    }
                } else {
                    view! {
                        <TextInput
                            value=value
                            placeholder=placeholder
                            name=name
                            id=field_id
                            required=required
                            disabled=disabled
                            state=state
                        />
                    }
                }
            }}
        </FormField>
    }
}

#[component]
pub fn TextAreaFormField(
    /// TextArea value
    #[prop(into)]
    value: Signal<String>,
    /// Label text
    label: String,
    /// TextArea placeholder
    #[prop(default = "".to_string())]
    placeholder: String,
    /// Number of rows
    #[prop(default = 3)]
    rows: u32,
    /// Maximum number of characters
    #[prop(optional)]
    max_length: Option<u32>,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Whether the textarea is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Optional error message
    #[prop(optional)]
    error: Option<String>,
    /// Optional help text
    #[prop(optional)]
    help: Option<String>,
    /// Input event handler
    #[prop(optional)]
    on_input: Option<Box<dyn Fn(Event) + Send + Sync>>,
    /// HTML name attribute
    #[prop(default = "".to_string())]
    name: String,
    /// Additional CSS classes for the container
    #[prop(default = "space-y-2".to_string())]
    class: String,
    /// Show character counter
    #[prop(default = false)]
    show_counter: bool,
) -> impl IntoView {
    let field_id = if name.is_empty() {
        format!("field_{}", label.to_lowercase().replace(" ", "_"))
    } else {
        name.clone()
    };
    
    let state = if error.is_some() { InputState::Error } else { InputState::Normal };
    
    view! {
        <FormField
            label=label
            required=required
            error=error.unwrap_or_default()
            help=help.unwrap_or_default()
            class=class
            id=field_id.clone()
        >
            {if show_counter && max_length.is_some() {
                if let Some(handler) = on_input {
                    view! {
                        <TextAreaWithCounter
                            value=value
                            max_length=max_length.unwrap()
                            placeholder=placeholder
                            name=name
                            id=field_id
                            rows=rows
                            required=required
                            disabled=disabled
                            state=state
                            on_input=handler
                        />
                    }.into_any()
                } else {
                    view! {
                        <TextAreaWithCounter
                            value=value
                            max_length=max_length.unwrap()
                            placeholder=placeholder
                            name=name
                            id=field_id
                            rows=rows
                            required=required
                            disabled=disabled
                            state=state
                        />
                    }.into_any()
                }
            } else {
                if let Some(handler) = on_input {
                    view! {
                        <TextArea
                            value=value
                            placeholder=placeholder
                            name=name
                            id=field_id
                            rows=rows
                            required=required
                            disabled=disabled
                            state=state
                            on_input=handler
                        />
                    }.into_any()
                } else {
                    view! {
                        <TextArea
                            value=value
                            placeholder=placeholder
                            name=name
                            id=field_id
                            rows=rows
                            required=required
                            disabled=disabled
                            state=state
                        />
                    }.into_any()
                }
            }}
        </FormField>
    }
}
