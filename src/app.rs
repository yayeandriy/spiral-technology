
use leptos::{html::A, logging, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, path, StaticSegment
};

use crate::{areas::areas_context::{AreaContextProvider, AreaRoute}, catalog::{catalog_context::{CatalogContextProvider, CatalogRoute}, views::catalog_editor::CatalogEditor}, pages::{about_page::AboutPage, catalog_page::CatalogPage, home_page::HomePage}, projects::{projects_context::{ProjectProvider, ProjectRoute}, views::{projects_editor::ProjectsEditor, projects_list::ProjectsList}}};



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
                                <CatalogPage />
                            </CatalogRoute>
                            </AreaRoute>
                        }}
                    >
                    
                        // <Route path=path!(":project_id")   
                        // view=|| view! { 
                        //    <ProjectRoute>
                        //          <ProjectsEditor />
                        //     </ProjectRoute>     
                              
                        // }/>

                        <Route path=path!("")  view=|| view! { 
                           <ProjectRoute>
                                 <ProjectsEditor />
                            </ProjectRoute>     
                              
                        }/>
                        <Route path=path!("areas") view=|| view! {
                            <AreaRoute>
                                <CatalogEditor />
                            </AreaRoute>
                        }/>

                        <Route path=path!("about") view=AboutPage/>
                    </ParentRoute>
                </Routes>   
            </Router>
        </ProjectProvider>   
        </AreaContextProvider>  
        </CatalogContextProvider>  
    }
}

