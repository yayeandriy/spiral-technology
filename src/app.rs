
use leptos::{html::A, logging, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, path, StaticSegment
};

use crate::{areas::areas_context::{AreaContextProvider, AreaRoute}, catalog::catalog_context::{CatalogContextProvider, CatalogRoute}, pages::{about_page::AboutPage, home_page::HomePage}, projects::{project_page::ProjectPage, projects_context::{ProjectProvider, ProjectRoute}, table_page::TablePage}};



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="icon" type_="image/png" href="/public/favicon.png" />
        <CatalogContextProvider>
        <AreaContextProvider>
        <ProjectProvider> 
            <Router>
                <Routes fallback=|| "Page not found.">
                 <ParentRoute 
                        path=StaticSegment("/") 
                        view=||{ view! {
                            <AreaRoute>
                            <CatalogRoute>
                                <HomePage />
                            </CatalogRoute>
                            </AreaRoute>
                        }}
                    >
                    
                        <Route path=path!(":project_id")   
                        view=|| view! { 
                           <ProjectRoute>
                                 <ProjectPage />
                            </ProjectRoute>     
                              
                        }/>

                        <Route path=path!("") view=TablePage/>
                        <Route path=path!("about") view=AboutPage/>
                    </ParentRoute>
                </Routes>   
            </Router>
        </ProjectProvider>   
        </AreaContextProvider>  
        </CatalogContextProvider>  
    }
}

