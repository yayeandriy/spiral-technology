use crate::auth::auth_context::use_auth;
use crate::ui::form::simple_form_input::SimpleFormInput;
use crate::ui::button::{Button, ButtonVariant};
use leptos::prelude::*;
use leptos::{ev, task::spawn_local};

#[component]
pub fn ResetPasswordForm() -> impl IntoView {
    let auth = use_auth();
    let (email, set_email) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (is_sent, set_is_sent) = signal(false);

    let auth_for_submit = auth.clone();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let auth = auth_for_submit.clone();
        let email_val = email.get();
        
        if email_val.is_empty() {
            auth.error.1.set(Some("Please enter your email address".to_string()));
            return;
        }

        set_is_submitting.set(true);
        
        spawn_local(async move {
            auth.reset_password(email_val).await;
            set_is_submitting.set(false);
            set_is_sent.set(true);
        });
    };

    view! {
        <div class="w-full max-w-md mx-auto">
            {move || {
                let on_submit = on_submit.clone();
                if is_sent.get() {
                    view! {
                        <div class="text-center space-y-6">
                            <h2 class="text-2xl font-bold text-gray-900">Check Your Email</h2>
                            <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded">
                                "We've sent a password reset link to your email address."
                            </div>
                            <p class="text-sm text-gray-600">
                                "Didn't receive the email? Check your spam folder or "
                                <button 
                                    type="button"
                                    class="text-blue-600 hover:text-blue-500 underline"
                                    on:click=move |_| {
                                        set_is_sent.set(false);
                                        set_email.set(String::new());
                                        auth.error.1.set(None);
                                    }
                                >
                                    "try again"
                                </button>
                            </p>
                            <div class="text-center">
                                <a href="/login" class="text-sm text-blue-600 hover:text-blue-500">
                                    "← Back to sign in"
                                </a>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <form on:submit=on_submit class="space-y-6">
                            <div>
                                <h2 class="text-2xl font-bold text-gray-900 mb-2">Reset Password</h2>
                                <p class="text-sm text-gray-600 mb-6">
                                    "Enter your email address and we'll send you a link to reset your password."
                                </p>
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
                                label="Email Address".to_string()
                                input_type="email".to_string()
                                value=email
                                on_input=Callback::new(move |val| set_email.set(val))
                                placeholder="Enter your email".to_string()
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
                                            "Sending reset link..."
                                        } else {
                                            "Send Reset Link"
                                        }
                                    }}
                                </Button>
                            </div>

                            <div class="text-center">
                                <a href="/login" class="text-sm text-blue-600 hover:text-blue-500">
                                    "← Back to sign in"
                                </a>
                            </div>
                        </form>
                    }.into_any()
                }
            }}
        </div>
    }
}
