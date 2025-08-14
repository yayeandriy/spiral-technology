use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};


 const SUPABASE_URL: &str = "https://kwedqsqfgiydxypbfntb.supabase.co";
pub const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imt3ZWRxc3FmZ2l5ZHh5cGJmbnRiIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDU4NTM0MDcsImV4cCI6MjA2MTQyOTQwN30._MkW4-EcPk85UiBWM3HdHmeEeqYkffWDhSlaRmnS-XY";


/// Shared function for GET-only calls.
async fn supabase_request(path: &str) -> Result<Response, String> {
    let url = format!("{}{}", SUPABASE_URL, path);
    Request::get(&url)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {}", SUPABASE_ANON_KEY))
        .send()
        .await
        .map_err(|e| e.to_string())
}


pub async fn supabase_get<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let resp = supabase_request(path).await?;
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
    let url = format!("{}{}", SUPABASE_URL, path);
    let body = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let resp = Request::post(&url)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {}", SUPABASE_ANON_KEY))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .header("Accept", "application/vnd.pgrst.object+json")
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
    let url = format!("{}{}", SUPABASE_URL, path);
    let body = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let resp = Request::patch(&url)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {}", SUPABASE_ANON_KEY))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .header("Accept", "application/vnd.pgrst.object+json")
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
    let url = format!("{}{}", SUPABASE_URL, path);
    let resp = Request::delete(&url)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", &format!("Bearer {}", SUPABASE_ANON_KEY))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if (200..300).contains(&resp.status()) {
        Ok(())
    } else {
        Err(format!("DELETE {} failed: HTTP {}", path, resp.status()))
    }
}