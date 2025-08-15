
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, path, StaticSegment
};

use crate::{areas::{areas_context::{AreaContextProvider, AreaRoute}, views::{areas_table::AreasTable, areas_editor::AreasEditor}}, catalog::{catalog_context::{CatalogContextProvider, CatalogRoute}}, pages::{about_page::AboutPage, editor_page::EditorPage, home_page::HomePage}, projects::{projects_context::{ProjectProvider, ProjectRoute}, views::projects_editor::ProjectsEditor}};



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
                 <Route path=path!("") view=|| view! {
                     <AreaRoute>
                     <CatalogRoute>
                     <ProjectRoute>
                         <HomePage />
                     </ProjectRoute>
                     </CatalogRoute>
                     </AreaRoute>
                 }/>
                 
                 <ParentRoute 
                        path=StaticSegment("/home") 
                        view=||{ view! {
                            <AreaRoute>
                            <CatalogRoute>
                            <ProjectRoute>
                                <HomePage />
                            </ProjectRoute>
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

                        // <Route path=path!("project-editor")  view=|| view! { 
                        //    <ProjectRoute>
                        //          <ProjectsEditor />
                        //     </ProjectRoute>     
                              
                        // }/>
                        // <Route path=path!("area-editor") view=|| view! {
                        //     <AreaRoute>
                        //         <AreasEditor />
                        //     </AreaRoute>
                        // }/>

                        <Route path=path!("") view=AreasTable/>
                        <Route path=path!("about") view=AboutPage/>
                    </ParentRoute>
                 <ParentRoute 
                        path=StaticSegment("/editor") 
                        view=||{ view! {
                            <AreaRoute>
                            <CatalogRoute>
                             <ProjectRoute>
                                <EditorPage />
                            </ProjectRoute>
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
                            <ProjectsEditor />
                            <AreasEditor />
                        }/>

                    </ParentRoute>
                </Routes>   
            </Router>
        </ProjectProvider>   
        </AreaContextProvider>  
        </CatalogContextProvider>  
    }
}

