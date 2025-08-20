use crate::auth::auth_context::use_auth;
use crate::ui::form::simple_form_input::SimpleFormInput;
use crate::ui::button::{Button, ButtonVariant};
use leptos::prelude::*;
use leptos::{ev, task::spawn_local};
use leptos_router::hooks::use_navigate;

#[component]
pub fn LoginForm(
    #[prop(optional)]
    show_register_link: Option<bool>,
) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);

    let show_register = show_register_link.unwrap_or(true);

    // Effect to handle navigation after successful login
    Effect::new(move |_| {
        if auth.is_authenticated.0.get() && !auth.is_loading.0.get() {
            navigate("/editor", Default::default());
        }
    });

    let on_submit = {
        let auth = auth.clone();
        move |ev: ev::SubmitEvent| {
            ev.prevent_default();
            
            let auth = auth.clone();
            let email_val = email.get();
            let password_val = password.get();
            
            if email_val.is_empty() || password_val.is_empty() {
                auth.error.1.set(Some("Please fill in all fields".to_string()));
                return;
            }

            set_is_submitting.set(true);
            
            spawn_local(async move {
                auth.sign_in(email_val, password_val).await;
                set_is_submitting.set(false);
            });
        }
    };

    view! {
        <div class="w-full max-w-md mx-auto">
            <form on:submit=on_submit class="space-y-6">
                <div>
                    <h2 class="text-2xl font-bold text-gray-900 mb-6">Sign In</h2>
                </div>
                
                {move || {
                    if let Some(error) = auth.error.0.get() {
                        view! {
                            <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                                {error}
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}

                <SimpleFormInput
                    id="email".to_string()
                    label="Email".to_string()
                    input_type="email".to_string()
                    value=email
                    on_input=Callback::new(move |val| set_email.set(val))
                    placeholder="Enter your email".to_string()
                    required=true
                />

                <SimpleFormInput
                    id="password".to_string()
                    label="Password".to_string()
                    input_type="password".to_string()
                    value=password
                    on_input=Callback::new(move |val| set_password.set(val))
                    placeholder="Enter your password".to_string()
                    required=true
                />

                <div class="flex items-center justify-between">
                    <Button
                        type_="submit".to_string()
                        variant=ButtonVariant::Primary
                        disabled=is_submitting.get() || auth.is_loading.0.get()
                        class="w-full".to_string()
                        on_click=move |_: leptos::ev::MouseEvent| {}
                    >
                        {move || {
                            if is_submitting.get() || auth.is_loading.0.get() {
                                "Signing in..."
                            } else {
                                "Sign In"
                            }
                        }}
                    </Button>
                </div>

                {move || {
                    if show_register {
                        view! {
                            <div class="text-center">
                                <p class="text-sm text-gray-600">
                                    "Don't have an account? "
                                    <a href="/register" class="font-medium text-blue-600 hover:text-blue-500">
                                        "Sign up"
                                    </a>
                                </p>
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}

                <div class="text-center">
                    <a href="/reset-password" class="text-sm text-blue-600 hover:text-blue-500">
                        "Forgot your password?"
                    </a>
                </div>
            </form>
        </div>
    }
}
