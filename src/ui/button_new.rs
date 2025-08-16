use leptos::prelude::*;

/// Button size variants
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-3 py-1.5 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-sm",
            ButtonSize::Large => "px-6 py-3 text-base",
        }
    }
}

/// Button style variants
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Cancel,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 text-white border-transparent",
            ButtonVariant::Secondary => "bg-white hover:bg-gray-50 text-gray-900 border-gray-300",
            ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white border-transparent",
            ButtonVariant::Cancel => "bg-white hover:bg-gray-50 text-gray-700 border-gray-300",
        }
    }
}

/// Main button component with signal-based reactivity
#[component]
pub fn Button<F>(
    /// Click handler
    on_click: F,
    /// Button variant
    #[prop(default = ButtonVariant::Primary)]
    variant: ButtonVariant,
    /// Button size
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Whether the button is disabled
    #[prop(default = Signal::derive(|| false))]
    disabled: Signal<bool>,
    /// Additional CSS classes
    #[prop(default = "".to_string())]
    class: String,
    /// Button content
    children: Children,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    let base_classes = "inline-flex items-center justify-center border font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors duration-200";
    
    view! {
        <button
            type="button"
            class=move || format!(
                "{} {} {} {} {}",
                base_classes,
                variant.classes(),
                size.classes(),
                if disabled.get() { "opacity-50 cursor-not-allowed" } else { "" },
                class
            )
            on:click=on_click
            disabled=disabled
        >
            {children()}
        </button>
    }
}

/// Primary button shorthand
#[component]
pub fn PrimaryButton<F>(
    on_click: F,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <Button
            variant=ButtonVariant::Primary
            size=size
            disabled=disabled
            class=class
            on_click=on_click
        >
            {children()}
        </Button>
    }
}

/// Secondary button shorthand
#[component]
pub fn SecondaryButton<F>(
    on_click: F,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <Button
            variant=ButtonVariant::Secondary
            size=size
            disabled=disabled
            class=class
            on_click=on_click
        >
            {children()}
        </Button>
    }
}

/// Cancel button shorthand
#[component]
pub fn CancelButton<F>(
    on_click: F,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <Button
            variant=ButtonVariant::Cancel
            size=size
            disabled=disabled
            class=class
            on_click=on_click
        >
            {children()}
        </Button>
    }
}

/// Danger button shorthand
#[component]
pub fn DangerButton<F>(
    on_click: F,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = "".to_string())] class: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <Button
            variant=ButtonVariant::Danger
            size=size
            disabled=disabled
            class=class
            on_click=on_click
        >
            {children()}
        </Button>
    }
}
