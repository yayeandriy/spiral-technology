use crate::auth::auth_context::use_auth;
use crate::auth::model::UserMetadata;
use crate::ui::form::simple_form_input::SimpleFormInput;
use crate::ui::button::{Button, ButtonVariant};
use leptos::prelude::*;
use leptos::{ev, task::spawn_local};

#[component]
pub fn RegisterForm(
    #[prop(optional)]
    show_login_link: Option<bool>,
) -> impl IntoView {
    let auth = use_auth();
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (full_name, set_full_name) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);

    let show_login = show_login_link.unwrap_or(true);

    let auth_clone = auth.clone();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let auth = auth_clone.clone();
        let email_val = email.get();
        let password_val = password.get();
        let confirm_password_val = confirm_password.get();
        let full_name_val = full_name.get();
        
        // Validation
        if email_val.is_empty() || password_val.is_empty() {
            auth.error.1.set(Some("Please fill in all required fields".to_string()));
            return;
        }

        if password_val.len() < 6 {
            auth.error.1.set(Some("Password must be at least 6 characters long".to_string()));
            return;
        }

        if password_val != confirm_password_val {
            auth.error.1.set(Some("Passwords do not match".to_string()));
            return;
        }

        set_is_submitting.set(true);
        
        let metadata = if !full_name_val.is_empty() {
            Some(UserMetadata {
                full_name: Some(full_name_val),
                ..Default::default()
            })
        } else {
            None
        };
        
        spawn_local(async move {
            auth.sign_up(email_val, password_val, metadata).await;
            set_is_submitting.set(false);
        });
    };

    view! {
        <div class="w-full max-w-md mx-auto">
            <form on:submit=on_submit class="space-y-6">
                <div>
                    <h2 class="text-2xl font-bold text-gray-900 mb-6">Create Account</h2>
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
                    id="full_name".to_string()
                    label="Full Name".to_string()
                    input_type="text".to_string()
                    value=full_name
                    on_input=Callback::new(move |val| set_full_name.set(val))
                    placeholder="Enter your full name (optional)".to_string()
                    required=false
                />

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
                    placeholder="Enter your password (min 6 characters)".to_string()
                    required=true
                />

                <SimpleFormInput
                    id="confirm_password".to_string()
                    label="Confirm Password".to_string()
                    input_type="password".to_string()
                    value=confirm_password
                    on_input=Callback::new(move |val| set_confirm_password.set(val))
                    placeholder="Confirm your password".to_string()
                    required=true
                />

                <div class="flex items-center justify-between">
                    <Button
                        type_="submit".to_string()
                        variant=ButtonVariant::Primary
                        disabled=is_submitting.get() || auth.is_loading.0.get()
                        class="w-full".to_string()
                        on_click=move |_| {}
                    >
                        {move || {
                            if is_submitting.get() || auth.is_loading.0.get() {
                                "Creating account..."
                            } else {
                                "Create Account"
                            }
                        }}
                    </Button>
                </div>

                {move || {
                    if show_login {
                        view! {
                            <div class="text-center">
                                <p class="text-sm text-gray-600">
                                    "Already have an account? "
                                    <a href="/login" class="font-medium text-blue-600 hover:text-blue-500">
                                        "Sign in"
                                    </a>
                                </p>
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </form>
        </div>
    }
}
