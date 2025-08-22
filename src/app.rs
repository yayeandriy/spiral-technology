
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes}, path, StaticSegment,
    hooks::use_navigate
};

use crate::{
    areas::{areas_context::{AreaContextProvider, AreaRoute}, views::areas_table::AreasTable}, 
    catalog::catalog_context::{CatalogContextProvider, CatalogRoute}, 
    content::content_context::{ProjectContentContextProvider, ProjectContentRoute}, 
    pages::{about_page::AboutPage, editor_page::EditorPage, home_page::HomePage}, 
    projects::{projects_context::{ProjectProvider, ProjectRoute}, views::{editor::project_edit_page::project_edit_page::ProjectEditPage, landing::project_view::ProjectView}},
    auth::{
        auth_context::AuthProvider,
        views::{
            auth_form::{AuthForm, AuthFormMode},
            reset_password_form::ResetPasswordForm,
            protected_route::ProtectedRoute,
        }
    }
};

#[component]
fn RedirectToHome() -> impl IntoView {
    let navigate = use_navigate();
    
    Effect::new(move |_| {
        navigate("/home", Default::default());
    });
    
    view! {
        <div></div>
    }
}



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="icon" type_="image/png" href="/public/favicon.png" />
        <AuthProvider>
        <CatalogContextProvider>
        <AreaContextProvider>
        <ProjectProvider> 
        <ProjectContentContextProvider> 
            <Router>
                <Routes fallback=|| "Page not found.">
                 // Auth routes (public)
                 <Route path=path!("/login") view=|| view! {
                     <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
                         <AuthForm mode=AuthFormMode::Login />
                     </div>
                 }/>
                 
                //  <Route path=path!("/register") view=|| view! {
                //      <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
                //          <AuthForm mode=AuthFormMode::Register />
                //      </div>
                //  }/>
                 
                 <Route path=path!("/reset-password") view=|| view! {
                     <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
                         <ResetPasswordForm />
                     </div>
                 }/>
                 
                 // Profile route (protected)
             
                 
                 <Route path=path!("") view=RedirectToHome/>
                 
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
                    

                        <Route path=path!("") view=AreasTable/>
                         <Route path=path!(":project_id")   
                            view=||{ 
                                
                                view! { 
                            <ProjectRoute>
                            <ProjectContentRoute>
                                    <ProjectView />
                            </ProjectContentRoute>
                            </ProjectRoute>     
                                
                            }}/>
                        <Route path=path!("about") view=AboutPage/>
                    </ParentRoute>
                 <ParentRoute 
                        path=StaticSegment("/editor") 
                        view=||{ view! {
                            <ProtectedRoute redirect_path="/login" >
                            <AreaRoute>
                            <CatalogRoute>
                             <ProjectRoute>
                                <EditorPage />
                            </ProjectRoute>
                            </CatalogRoute>
                            </AreaRoute>
                            </ProtectedRoute>
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
        </AuthProvider>
    }
}

