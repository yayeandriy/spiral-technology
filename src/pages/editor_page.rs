use leptos::prelude::*;
use leptos_router::components::Outlet;


#[component]
pub fn EditorPage() -> impl IntoView {
    view! {
        <main class="w-full h-screen bg-white flex items-start justify-start p-8 text-[20px]" style="line-height: 1.5;">
         <div class="flex w-full">
            <Outlet />
         </div> 
        </main>
    }
}
