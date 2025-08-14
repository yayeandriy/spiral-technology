use wasm_bindgen::prelude::*;
use crate::supabase::{SupabaseConfig, supabase_get_with_config};
use serde_json::Value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn test_supabase_keys() {
    console_log!("🔍 Testing Supabase API key configurations...");
    
    // Test 1: Current configuration (should be old JWT keys)
    console_log!("\n1️⃣ Testing current configuration:");
    let current_config = SupabaseConfig::new();
    console_log!("Key type: {}", if current_config.is_jwt_key() { "JWT" } else { "API Key" });
    console_log!("Key starts with: {}...", &current_config.api_key[..10]);
    
    match test_areas_endpoint(&current_config).await {
        Ok(msg) => console_log!("✅ {}", msg),
        Err(msg) => console_log!("❌ {}", msg),
    }
    
    // Test 2: New API keys
    console_log!("\n2️⃣ Testing new API keys:");
    let new_config = SupabaseConfig::with_new_keys();
    console_log!("Key type: {}", if new_config.is_jwt_key() { "JWT" } else { "API Key" });
    console_log!("Key starts with: {}...", &new_config.api_key[..10]);
    
    match test_areas_endpoint(&new_config).await {
        Ok(msg) => console_log!("✅ {}", msg),
        Err(msg) => console_log!("❌ {}", msg),
    }
    
    // Test 3: Simple areas request without filters
    console_log!("\n3️⃣ Testing simple areas request (no filters):");
    match supabase_get_with_config::<Value>("/rest/v1/areas?select=*", &current_config).await {
        Ok(data) => {
            let count = data.as_array().map(|arr| arr.len()).unwrap_or(1);
            console_log!("✅ Simple request successful: {} records", count);
        },
        Err(e) => console_log!("❌ Simple request failed: {}", e),
    }
    
    console_log!("\n🎯 Test completed!");
}

async fn test_areas_endpoint(config: &SupabaseConfig) -> Result<String, String> {
    let path = "/rest/v1/areas?select=*&host=eq.roboscope";
    match supabase_get_with_config::<Value>(path, config).await {
        Ok(data) => {
            let count = data.as_array().map(|arr| arr.len()).unwrap_or(1);
            Ok(format!("Areas request successful: {} records found", count))
        },
        Err(e) => Err(format!("Areas request failed: {}", e)),
    }
}
