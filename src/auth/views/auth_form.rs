use crate::auth::views::{login_form::LoginForm, register_form::RegisterForm};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AuthFormMode {
    Login,
    Register,
}

#[component]
pub fn AuthForm(
    #[prop(optional)]
    mode: Option<AuthFormMode>,
    #[prop(optional)]
    show_mode_toggle: Option<bool>,
) -> impl IntoView {
    let (current_mode, set_current_mode) = signal(mode.unwrap_or(AuthFormMode::Login));
    let show_toggle = show_mode_toggle.unwrap_or(true);

    view! {
        <div class="w-full max-w-md mx-auto">
            {move || {
                match current_mode.get() {
                    AuthFormMode::Login => view! {
                        <LoginForm
                            show_register_link=show_toggle
                        />
                        
                        {move || {
                            if show_toggle {
                                view! {
                                    <div class="mt-6 text-center">
                                        <button
                                            type="button"
                                            class="text-sm text-blue-600 hover:text-blue-500"
                                            on:click=move |_| set_current_mode.set(AuthFormMode::Register)
                                        >
                                            "Don't have an account? Sign up"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }
                        }}
                    }.into_any(),
                    
                    AuthFormMode::Register => view! {
                        <RegisterForm
                            show_login_link=show_toggle
                        />
                        
                        {move || {
                            if show_toggle {
                                view! {
                                    <div class="mt-6 text-center">
                                        <button
                                            type="button"
                                            class="text-sm text-blue-600 hover:text-blue-500"
                                            on:click=move |_| set_current_mode.set(AuthFormMode::Login)
                                        >
                                            "Already have an account? Sign in"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }
                        }}
                    }.into_any(),
                }
            }}
        </div>
    }
}
