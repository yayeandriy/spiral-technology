use crate::supabase::{SupabaseConfig, supabase_get_with_config, test_supabase_connection};
use serde_json::Value;

/// Test the specific areas endpoint that's failing
pub async fn debug_areas_request() -> Result<String, String> {
    let config = SupabaseConfig::new();
    
    let mut debug_info = Vec::new();
    
    // 1. Test basic connection
    match test_supabase_connection().await {
        Ok(info) => debug_info.push(format!("✓ Connection test: {}", info)),
        Err(e) => debug_info.push(format!("✗ Connection test failed: {}", e)),
    }
    
    // 2. Test the problematic areas endpoint
    let areas_path = "/rest/v1/areas?select=*&host=eq.roboscope";
    match supabase_get_with_config::<Value>(areas_path, &config).await {
        Ok(data) => debug_info.push(format!("✓ Areas request successful: {:?}", data)),
        Err(e) => debug_info.push(format!("✗ Areas request failed: {}", e)),
    }
    
    // 3. Test a simpler areas request without filters
    let simple_areas_path = "/rest/v1/areas?select=*";
    match supabase_get_with_config::<Value>(simple_areas_path, &config).await {
        Ok(data) => debug_info.push(format!("✓ Simple areas request successful: {} records", 
            data.as_array().map(|arr| arr.len()).unwrap_or(1))),
        Err(e) => debug_info.push(format!("✗ Simple areas request failed: {}", e)),
    }
    
    // 4. Try with old JWT keys for comparison
    let old_config = SupabaseConfig::with_old_keys();
    match supabase_get_with_config::<Value>(areas_path, &old_config).await {
        Ok(data) => debug_info.push(format!("✓ Areas request with old keys successful: {:?}", data)),
        Err(e) => debug_info.push(format!("✗ Areas request with old keys failed: {}", e)),
    }
    
    Ok(debug_info.join("\n"))
}

/// Test different API key configurations
pub async fn test_api_key_configs() -> Result<String, String> {
    let mut results = Vec::new();
    
    // Test new API keys
    let new_config = SupabaseConfig::with_new_keys();
    let new_result = match test_supabase_connection().await {
        Ok(info) => format!("New API Keys: ✓ {}", info),
        Err(e) => format!("New API Keys: ✗ {}", e),
    };
    results.push(new_result);
    
    // Test old JWT keys
    let old_config = SupabaseConfig::with_old_keys();
    let old_result = match test_supabase_connection().await {
        Ok(info) => format!("Old JWT Keys: ✓ {}", info),
        Err(e) => format!("Old JWT Keys: ✗ {}", e),
    };
    results.push(old_result);
    
    Ok(results.join("\n"))
}
