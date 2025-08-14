use leptos::prelude::*;
use leptos_router::components::Outlet;


#[component]
pub fn CatalogPage() -> impl IntoView {
    view! {
        <main class="w-full h-screen flex items-start justify-start p-8 text-[20px]" style="line-height: 1.5;">
         <div></div> 
         <Outlet />
        </main>
    }
}
