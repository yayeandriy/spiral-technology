
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, path, StaticSegment
};

use crate::pages::home_page::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="icon" type_="image/png" href="/public/favicon.png" />
        
            <Router>
                <Routes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=HomePage/>                    
                </Routes>          
            </Router>
        
    }
}

