use crate::auth::model::{
    User, Session, AuthResponse, SignInRequest, SignUpRequest, 
    ResetPasswordRequest, UpdateUserRequest, AuthError, LocalSession
};
use crate::supabase::SupabaseConfig;
use leptos::{
    logging,
    prelude::{
        provide_context,
        signal,
        use_context,
        Children,
        ReadSignal,
        Set,
        WriteSignal,
        Get,
    },
    *,
};
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde_json;

const AUTH_STORAGE_KEY: &str = "supabase_auth";
const LOCAL_SESSION_KEY: &str = "local_auth_session";

#[derive(Clone)]
pub struct AuthContext {
    pub user: (ReadSignal<Option<User>>, WriteSignal<Option<User>>),
    pub session: (ReadSignal<Option<Session>>, WriteSignal<Option<Session>>),
    pub is_loading: (ReadSignal<bool>, WriteSignal<bool>),
    pub error: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub is_authenticated: (ReadSignal<bool>, WriteSignal<bool>),
}

impl AuthContext {
    pub fn new() -> Self {
        let context = Self {
            user: signal::<Option<User>>(None),
            session: signal::<Option<Session>>(None),
            is_loading: signal(false),
            error: signal(None),
            is_authenticated: signal(false),
        };
        
        // Try to restore session from localStorage
        context.restore_session();
        
        context
    }

    fn restore_session(&self) {
        // First try to restore a Supabase session
        if let Ok(stored_session) = LocalStorage::get::<Session>(AUTH_STORAGE_KEY) {
            // Check if session is still valid (not expired)
            let now = js_sys::Date::now() as i64 / 1000; // Current time in seconds
            if stored_session.expires_at > now {
                self.session.1.set(Some(stored_session.clone()));
                self.user.1.set(Some(stored_session.user));
                self.is_authenticated.1.set(true);
                logging::log!("Supabase session restored from localStorage");
                return;
            } else {
                // Session expired, remove it
                LocalStorage::delete(AUTH_STORAGE_KEY);
                logging::log!("Stored Supabase session expired, cleared");
            }
        }
        
        // If no Supabase session, try to restore a local session
        if let Some(local_session) = self.load_local_session() {
            let now = js_sys::Date::now() / 1000.0; // Current time in seconds
            if local_session.expires_at > now as i64 {
                self.user.1.set(Some(local_session.user));
                self.is_authenticated.1.set(true);
                logging::log!("Local session restored from localStorage");
            } else {
                // Local session expired, remove it
                LocalStorage::delete(LOCAL_SESSION_KEY);
                logging::log!("Stored local session expired, cleared");
            }
        }
    }

    fn store_session(&self, session: &Session) {
        if let Err(e) = LocalStorage::set(AUTH_STORAGE_KEY, session) {
            logging::log!("Failed to store session: {:?}", e);
        }
    }

    fn store_local_session(&self, local_session: &LocalSession) {
        if let Err(e) = LocalStorage::set(LOCAL_SESSION_KEY, local_session) {
            logging::log!("Failed to store local session: {:?}", e);
        }
    }

    fn load_local_session(&self) -> Option<LocalSession> {
        match LocalStorage::get(LOCAL_SESSION_KEY) {
            Ok(local_session) => Some(local_session),
            Err(_) => None,
        }
    }

    fn clear_session(&self) {
        LocalStorage::delete(AUTH_STORAGE_KEY);
        LocalStorage::delete(LOCAL_SESSION_KEY);
        self.session.1.set(None);
        self.user.1.set(None);
        self.is_authenticated.1.set(false);
    }

    pub async fn sign_in(&self, email: String, password: String) {
        self.is_loading.1.set(true);
        self.error.1.set(None);

        let request = SignInRequest { email, password };
        
        match self.auth_request::<AuthResponse, SignInRequest>("/auth/v1/token?grant_type=password", &request).await {
            Ok(auth_response) => {
                if let Some(session) = auth_response.session {
                    self.store_session(&session);
                    self.session.1.set(Some(session.clone()));
                    self.user.1.set(Some(session.user));
                    self.is_authenticated.1.set(true);
                    logging::log!("Sign in successful");
                } else if let Some(user) = auth_response.user {
                    // User exists and email is confirmed, but no session was created
                    // Create our own local session instead of relying on Supabase
                    logging::log!("User confirmed but no Supabase session. Creating local session...");
                    
                    if user.email_confirmed_at.is_some() {
                        // Create a local session that expires in 24 hours (using js timestamp)
                        let now = js_sys::Date::now() / 1000.0; // Convert to seconds
                        let local_session = LocalSession {
                            user: user.clone(),
                            created_at: now as i64,
                            expires_at: (now + (24.0 * 60.0 * 60.0)) as i64, // 24 hours
                        };
                        
                        // Store the local session
                        self.store_local_session(&local_session);
                        
                        // Set the auth state
                        self.user.1.set(Some(user));
                        self.is_authenticated.1.set(true);
                        
                        logging::log!("Local session created successfully. User is now authenticated.");
                        self.error.1.set(None); // Clear any previous errors
                    } else {
                        logging::log!("Sign in failed: Email not confirmed");
                        self.error.1.set(Some("Please check your email and confirm your account before signing in.".to_string()));
                    }
                } else {
                    logging::log!("Sign in failed: No session or user returned");
                    self.error.1.set(Some("Invalid email or password".to_string()));
                }
            }
            Err(err) => {
                logging::log!("Sign in error: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.set(false);
    }

    pub async fn sign_up(&self, email: String, password: String, metadata: Option<crate::auth::model::UserMetadata>) {
        self.is_loading.1.set(true);
        self.error.1.set(None);

        let request = SignUpRequest { 
            email, 
            password, 
            data: metadata 
        };
        
        match self.auth_request::<AuthResponse, SignUpRequest>("/auth/v1/signup", &request).await {
            Ok(auth_response) => {
                if let Some(session) = auth_response.session {
                    self.store_session(&session);
                    self.session.1.set(Some(session.clone()));
                    self.user.1.set(Some(session.user));
                    self.is_authenticated.1.set(true);
                    logging::log!("Sign up successful");
                } else if let Some(_user) = auth_response.user {
                    // User created but needs email confirmation
                    logging::log!("Sign up successful, please check your email for confirmation");
                    self.error.1.set(Some("Please check your email for confirmation".to_string()));
                } else {
                    self.error.1.set(Some("Invalid response from server".to_string()));
                }
            }
            Err(err) => {
                logging::log!("Sign up error: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.set(false);
    }

    pub async fn sign_out(&self) {
        self.is_loading.1.set(true);
        self.error.1.set(None);

        // Call the logout endpoint
        if let Some(session) = self.session.0.get() {
            let _: Result<serde_json::Value, String> = self.auth_request_with_token("/auth/v1/logout", &serde_json::json!({}), &session.access_token).await;
        }

        self.clear_session();
        logging::log!("Signed out successfully");
        self.is_loading.1.set(false);
    }

    pub async fn reset_password(&self, email: String) {
        self.is_loading.1.set(true);
        self.error.1.set(None);

        let request = ResetPasswordRequest { email };
        
        match self.auth_request::<serde_json::Value, ResetPasswordRequest>("/auth/v1/recover", &request).await {
            Ok(_) => {
                logging::log!("Password reset email sent");
                self.error.1.set(Some("Password reset email sent. Please check your inbox.".to_string()));
            }
            Err(err) => {
                logging::log!("Password reset error: {}", err);
                self.error.1.set(Some(err));
            }
        }
        
        self.is_loading.1.set(false);
    }

    pub async fn update_user(&self, update_request: UpdateUserRequest) {
        self.is_loading.1.set(true);
        self.error.1.set(None);

        if let Some(session) = self.session.0.get() {
            match self.auth_request_with_token::<AuthResponse, UpdateUserRequest>("/auth/v1/user", &update_request, &session.access_token).await {
                Ok(auth_response) => {
                    if let Some(user) = auth_response.user {
                        self.user.1.set(Some(user));
                        logging::log!("User updated successfully");
                    }
                }
                Err(err) => {
                    logging::log!("Update user error: {}", err);
                    self.error.1.set(Some(err));
                }
            }
        } else {
            self.error.1.set(Some("No active session".to_string()));
        }
        
        self.is_loading.1.set(false);
    }

    pub async fn refresh_session(&self) {
        if let Some(session) = self.session.0.get() {
            self.is_loading.1.set(true);
            
            let refresh_request = serde_json::json!({
                "refresh_token": session.refresh_token
            });
            
            match self.auth_request::<AuthResponse, serde_json::Value>("/auth/v1/token?grant_type=refresh_token", &refresh_request).await {
                Ok(auth_response) => {
                    if let Some(new_session) = auth_response.session {
                        self.store_session(&new_session);
                        self.session.1.set(Some(new_session.clone()));
                        self.user.1.set(Some(new_session.user));
                        logging::log!("Session refreshed successfully");
                    }
                }
                Err(err) => {
                    logging::log!("Session refresh error: {}", err);
                    self.clear_session();
                }
            }
            
            self.is_loading.1.set(false);
        }
    }

    async fn auth_request<T, U>(&self, path: &str, payload: &U) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
        U: serde::Serialize,
    {
        let config = SupabaseConfig::new();
        let url = format!("{}{}", config.url, path);
        
        let request = Request::post(&url)
            .header("apikey", &config.api_key)
            .header("Content-Type", "application/json")
            .json(payload)
            .map_err(|e| e.to_string())?;

        let response = request.send().await.map_err(|e| e.to_string())?;
        
        if response.ok() {
            response.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            if let Ok(auth_error) = serde_json::from_str::<AuthError>(&error_text) {
                Err(auth_error.message)
            } else {
                Err(format!("HTTP {}: {}", response.status(), error_text))
            }
        }
    }

    async fn auth_request_with_token<T, U>(&self, path: &str, payload: &U, token: &str) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
        U: serde::Serialize,
    {
        let config = SupabaseConfig::new();
        let url = format!("{}{}", config.url, path);
        
        let request = Request::put(&url)
            .header("apikey", &config.api_key)
            .header("Authorization", &format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(payload)
            .map_err(|e| e.to_string())?;

        let response = request.send().await.map_err(|e| e.to_string())?;
        
        if response.ok() {
            response.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            if let Ok(auth_error) = serde_json::from_str::<AuthError>(&error_text) {
                Err(auth_error.message)
            } else {
                Err(format!("HTTP {}: {}", response.status(), error_text))
            }
        }
    }

    pub fn get_current_user(&self) -> Option<User> {
        self.user.0.get()
    }

    pub fn get_current_session(&self) -> Option<Session> {
        self.session.0.get()
    }

    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated.0.get()
    }
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let auth_context = AuthContext::new();
    provide_context(auth_context);
    children()
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext must be provided")
}
