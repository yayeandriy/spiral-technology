
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, hooks::{use_params, use_params_map}, path, StaticSegment
};

use crate::{areas::{areas_context::{AreaContextProvider, AreaRoute}, views::areas_table::AreasTable}, catalog::catalog_context::{CatalogContextProvider, CatalogRoute}, content::content_context::{ProjectContentContextProvider, ProjectContentRoute}, pages::{about_page::AboutPage, editor_page::EditorPage, home_page::HomePage}, projects::{projects_context::{ProjectProvider, ProjectRoute, ProjectURLParams}, views::{project_edit_page::ProjectEditPage, projects_editor::ProjectsEditor}}};



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="icon" type_="image/png" href="/public/favicon.png" />
        <CatalogContextProvider>
        <AreaContextProvider>
        <ProjectProvider> 
        <ProjectContentContextProvider> 
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
                    
                        <Route path=path!(":project_id")   
                        view=||{ 
                            
                            view! { 
                           <ProjectRoute>
                           <ProjectContentRoute>
                                 <ProjectEditPage />
                            </ProjectContentRoute>
                           </ProjectRoute>     
                              
                        }}/>

                        <Route path=path!("")  view=|| view! { 
                            <div />
                        }/>

                    </ParentRoute>
                </Routes>   
            </Router>
        </ProjectContentContextProvider>   
        </ProjectProvider>   
        </AreaContextProvider>  
        </CatalogContextProvider>  
    }
}

