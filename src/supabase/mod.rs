use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};

mod config;
pub use config::SupabaseConfig;

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
pub use debug::{debug_areas_request, test_api_key_configs};


/// Shared function for GET-only calls.
async fn supabase_request(path: &str, config: &SupabaseConfig) -> Result<Response, String> {
    let url = format!("{}{}", config.url, path);
    let mut request = Request::get(&url)
        .header("apikey", &config.api_key);
    
    // Add Authorization header for JWT-based keys
    if config.needs_auth_header() {
        request = request.header("Authorization", &format!("Bearer {}", config.api_key));
    }
    
    request.send()
        .await
        .map_err(|e| e.to_string())
}


pub async fn supabase_get<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let config = SupabaseConfig::new();
    let resp = supabase_request(path, &config).await?;
    if resp.status() == 200 {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("GET {} failed: HTTP {}", path, resp.status()))
    }
}

pub async fn supabase_post<T, U>(path: &str, payload: &U) -> Result<T, String>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let config = SupabaseConfig::new();
    let url = format!("{}{}", config.url, path);
    let body = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    
    let mut request = Request::post(&url)
        .header("apikey", &config.api_key)
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .header("Accept", "application/vnd.pgrst.object+json");
    
    // Add Authorization header for JWT-based keys
    if config.needs_auth_header() {
        request = request.header("Authorization", &format!("Bearer {}", config.api_key));
    }
    
    let resp = request
        .body(body).map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 201 || resp.status() == 200 {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("POST {} failed: HTTP {}", path, resp.status()))
    }
}

pub async fn supabase_patch<T, U>(path: &str, payload: &U) -> Result<T, String>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let config = SupabaseConfig::new();
    let url = format!("{}{}", config.url, path);
    let body = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    
    let mut request = Request::patch(&url)
        .header("apikey", &config.api_key)
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .header("Accept", "application/vnd.pgrst.object+json");
    
    // Add Authorization header for JWT-based keys
    if config.needs_auth_header() {
        request = request.header("Authorization", &format!("Bearer {}", config.api_key));
    }
    
    let resp = request
        .body(body).map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 200 {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("PATCH {} failed: HTTP {}", path, resp.status()))
    }
}

pub async fn supabase_delete(path: &str) -> Result<(), String> {
    let config = SupabaseConfig::new();
    let url = format!("{}{}", config.url, path);
    
    let mut request = Request::delete(&url)
        .header("apikey", &config.api_key);
    
    // Add Authorization header for JWT-based keys
    if config.needs_auth_header() {
        request = request.header("Authorization", &format!("Bearer {}", config.api_key));
    }
    
    let resp = request
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if (200..300).contains(&resp.status()) {
        Ok(())
    } else {
        Err(format!("DELETE {} failed: HTTP {}", path, resp.status()))
    }
}

// Utility functions for the new API key system

/// Create a Supabase client configuration with publishable key
pub fn with_publishable_key(publishable_key: &str) -> SupabaseConfig {
    SupabaseConfig::with_keys(Some(publishable_key), None, true)
}

/// Create a Supabase client configuration with secret key (for server-side operations)
pub fn with_secret_key(secret_key: &str) -> SupabaseConfig {
    SupabaseConfig::with_keys(None, Some(secret_key), true)
}

/// Advanced API functions that accept custom configuration

pub async fn supabase_get_with_config<T>(path: &str, config: &SupabaseConfig) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let resp = supabase_request(path, config).await?;
    if resp.status() == 200 {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("GET {} failed: HTTP {}", path, resp.status()))
    }
}

pub async fn supabase_post_with_config<T, U>(path: &str, payload: &U, config: &SupabaseConfig) -> Result<T, String>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let url = format!("{}{}", config.url, path);
    let body = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    
    let mut request = Request::post(&url)
        .header("apikey", &config.api_key)
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .header("Accept", "application/vnd.pgrst.object+json");
    
    // Add Authorization header for JWT-based keys
    if config.needs_auth_header() {
        request = request.header("Authorization", &format!("Bearer {}", config.api_key));
    }
    
    let resp = request
        .body(body).map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 201 || resp.status() == 200 {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("POST {} failed: HTTP {}", path, resp.status()))
    }
}

/// Debug function to test API key configuration
pub async fn test_supabase_connection() -> Result<String, String> {
    let config = SupabaseConfig::new();
    
    // Log configuration details for debugging
    let key_type = if config.is_jwt_key() { "JWT" } else { "API Key" };
    let auth_header = if config.needs_auth_header() { "Yes" } else { "No" };
    
    // Try a simple request to test the connection
    let test_path = "/rest/v1/";
    let resp = supabase_request(test_path, &config).await?;
    
    Ok(format!(
        "Connection test:\nStatus: {}\nKey type: {}\nAuth header: {}\nAPI Key (first 10 chars): {}...",
        resp.status(),
        key_type,
        auth_header,
        &config.api_key.chars().take(10).collect::<String>()
    ))
}