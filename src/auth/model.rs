use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub email_confirmed_at: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_sign_in_at: Option<String>,
    pub app_metadata: Option<AppMetadata>,
    pub user_metadata: Option<UserMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppMetadata {
    pub provider: Option<String>,
    pub providers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserMetadata {
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub full_name: Option<String>,
    pub iss: Option<String>,
    pub name: Option<String>,
    pub phone_verified: Option<bool>,
    pub picture: Option<String>,
    pub provider_id: Option<String>,
    pub sub: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub expires_at: i64,
    pub refresh_token: String,
    pub user: User,
}

// Local session for when we don't get a proper Supabase session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalSession {
    pub user: User,
    pub created_at: i64, // timestamp when session was created
    pub expires_at: i64, // when this local session expires (24 hours default)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthResponse {
    pub user: Option<User>,
    pub session: Option<Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub data: Option<UserMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePasswordRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub data: Option<UserMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthError {
    pub message: String,
    pub error_description: Option<String>,
}

impl User {
    pub fn display_name(&self) -> String {
        if let Some(ref metadata) = self.user_metadata {
            if let Some(ref full_name) = metadata.full_name {
                return full_name.clone();
            }
            if let Some(ref name) = metadata.name {
                return name.clone();
            }
        }
        self.email.clone()
    }

    pub fn avatar_url(&self) -> Option<String> {
        self.user_metadata
            .as_ref()
            .and_then(|metadata| metadata.avatar_url.clone())
            .or_else(|| self.user_metadata
                .as_ref()
                .and_then(|metadata| metadata.picture.clone()))
    }
}

impl Default for SignInRequest {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }
}

impl Default for SignUpRequest {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            data: None,
        }
    }
}

impl Default for ResetPasswordRequest {
    fn default() -> Self {
        Self {
            email: String::new(),
        }
    }
}

impl Default for UserMetadata {
    fn default() -> Self {
        Self {
            avatar_url: None,
            email: None,
            email_verified: None,
            full_name: None,
            iss: None,
            name: None,
            phone_verified: None,
            picture: None,
            provider_id: None,
            sub: None,
        }
    }
}
