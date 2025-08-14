use leptos::prelude::*;
use leptos::ev::MouseEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-blue-600 text-white border-transparent hover:bg-blue-700 focus:ring-blue-500",
            ButtonVariant::Secondary => "bg-blue-100 text-blue-700 border-transparent hover:bg-blue-200 focus:ring-blue-500",
            ButtonVariant::Danger => "bg-red-100 text-red-700 border-transparent hover:bg-red-200 focus:ring-red-500",
            ButtonVariant::Cancel => "bg-white text-gray-700 border-gray-300 hover:bg-gray-50 focus:ring-blue-500",
        }
    }
}

impl ButtonSize {
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-3 py-1 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-sm",
            ButtonSize::Large => "px-6 py-3 text-base",
        }
    }
}

#[component]
pub fn Button(
    /// The button content
    children: Children,
    /// Button variant (Primary, Secondary, Danger, Cancel)
    #[prop(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    /// Button size (Small, Medium, Large)
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Whether the button is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Button type (button, submit, reset)
    #[prop(default = "button".to_string())]
    type_: String,
    /// Click event handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(MouseEvent)>>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let base_classes = "font-medium border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200";
    
    let classes = format!(
        "{} {} {} {}",
        base_classes,
        variant.classes(),
        size.classes(),
        class
    );

    view! {
        <button
            type=type_
            class=classes
            disabled=disabled
            on:click=move |ev| {
                if let Some(ref handler) = on_click {
                    handler(ev);
                }
            }
        >
            {children()}
        </button>
    }
}

// Convenience components for common button types
#[component]
pub fn PrimaryButton(
    children: Children,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "button".to_string())] type_: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
    #[prop(default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_click {
            view! {
                <Button
                    variant=ButtonVariant::Primary
                    size=size
                    disabled=disabled
                    type_=type_
                    on_click=handler
                    class=class
                >
                    {children()}
                </Button>
            }
        } else {
            view! {
                <Button
                    variant=ButtonVariant::Primary
                    size=size
                    disabled=disabled
                    type_=type_
                    class=class
                >
                    {children()}
                </Button>
            }
        }}
    }
}

#[component]
pub fn SecondaryButton(
    children: Children,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "button".to_string())] type_: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
    #[prop(default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_click {
            view! {
                <Button
                    variant=ButtonVariant::Secondary
                    size=size
                    disabled=disabled
                    type_=type_
                    on_click=handler
                    class=class
                >
                    {children()}
                </Button>
            }
        } else {
            view! {
                <Button
                    variant=ButtonVariant::Secondary
                    size=size
                    disabled=disabled
                    type_=type_
                    class=class
                >
                    {children()}
                </Button>
            }
        }}
    }
}

#[component]
pub fn DangerButton(
    children: Children,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "button".to_string())] type_: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
    #[prop(default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_click {
            view! {
                <Button
                    variant=ButtonVariant::Danger
                    size=size
                    disabled=disabled
                    type_=type_
                    on_click=handler
                    class=class
                >
                    {children()}
                </Button>
            }
        } else {
            view! {
                <Button
                    variant=ButtonVariant::Danger
                    size=size
                    disabled=disabled
                    type_=type_
                    class=class
                >
                    {children()}
                </Button>
            }
        }}
    }
}

#[component]
pub fn CancelButton(
    children: Children,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "button".to_string())] type_: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
    #[prop(default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        {if let Some(handler) = on_click {
            view! {
                <Button
                    variant=ButtonVariant::Cancel
                    size=size
                    disabled=disabled
                    type_=type_
                    on_click=handler
                    class=class
                >
                    {children()}
                </Button>
            }
        } else {
            view! {
                <Button
                    variant=ButtonVariant::Cancel
                    size=size
                    disabled=disabled
                    type_=type_
                    class=class
                >
                    {children()}
                </Button>
            }
        }}
    }
}
