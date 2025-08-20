use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::auth::auth_context::use_auth;

#[component]
pub fn ProtectedRoute(
    #[prop(optional)] redirect_path: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let redirect = redirect_path.unwrap_or("/login");

    // Effect to handle redirects when authentication state changes
    Effect::new({
        let navigate = navigate.clone();
        let redirect = redirect.to_string();
        move |_| {
            if !auth.is_loading.0.get() && !auth.is_authenticated() {
                navigate(&redirect, Default::default());
            }
        }
    });

    // Effect::new(move |_| {

    //     if !is_loading.0.get() && !is_authenticated.get() {
    //         navigate(redirect, Default::default());
    //     }
    //     if !is_loading.0.get() && is_authenticated.get() && is_admin_route.unwrap_or(false) && !is_admin.get() {
    //         navigate(redirect, Default::default());
    //     }

    //     logging::log!("ProtectedRoute: is_loading: {}, is_authenticated: {}, is_admin: {}", 
    //         is_loading.0.get(), 
    //         is_authenticated.get(), 
    //         is_admin.get()
    //     );
       
    // });
    view! {              
        {children()}
    }
}