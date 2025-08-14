# Supabase API Key Migration Guide

This guide will help you migrate from the old JWT-based API keys to the new publishable/secret key system in your Rust WASM application.

## Current Status

✅ **Updated**: Supabase module now supports both old and new API key systems
✅ **Updated**: Configuration system to handle different key types
✅ **Added**: Debug utilities to test connections

## What's Changed

### Before (Old JWT System)
```rust
// Old hardcoded approach
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

// Headers sent:
// - apikey: {JWT_TOKEN}
// - Authorization: Bearer {JWT_TOKEN}
```

### After (New API Key System)
```rust
// New configurable approach
let config = SupabaseConfig::new(); // Uses publishable key

// Headers sent:
// - apikey: sb_publishable_...
// - Authorization: (not sent for new keys)
```

## Key Differences

| Aspect | Old JWT Keys | New API Keys |
|--------|-------------|--------------|
| **Format** | `eyJ...` (JWT) | `sb_publishable_...` |
| **Authorization Header** | Required (`Bearer {token}`) | Not required |
| **Rotation** | Requires JWT secret rotation | Individual key rotation |
| **Security** | Tied to JWT secret | Independent rotation |

## Troubleshooting the 400 Error

The 400 Bad Request error you're seeing with:
```
https://kwedqsqfgiydxypbfntb.supabase.co/rest/v1/areas?select=*&host=eq.roboscope
```

Could be caused by:

1. **New API Key Authentication**: The new keys have different authentication headers
2. **URL Encoding**: The query parameter might need encoding
3. **API Key Permissions**: New keys might have different permissions

## Testing Steps

1. **Test both key systems**:
   ```rust
   // Test new keys
   let new_config = SupabaseConfig::with_new_keys();
   let result = supabase_get_with_config::<Vec<Area>>("/rest/v1/areas", &new_config).await;
   
   // Test old keys  
   let old_config = SupabaseConfig::with_old_keys();
   let result = supabase_get_with_config::<Vec<Area>>("/rest/v1/areas", &old_config).await;
   ```

2. **Test without filters first**:
   ```rust
   // Simple request without query parameters
   let simple_path = "/rest/v1/areas?select=*";
   ```

3. **Test with URL encoding**:
   ```rust
   use serde_urlencoded;
   
   let encoded_query = serde_urlencoded::to_string([
       ("select", "*"),
       ("host", "eq.roboscope"),
   ]).unwrap();
   let path = format!("/rest/v1/areas?{}", encoded_query);
   ```

## Recommended Migration Path

1. **Phase 1**: Test with both key systems (current setup)
2. **Phase 2**: Identify which system works for your specific query
3. **Phase 3**: Update configuration to use working system
4. **Phase 4**: Test all endpoints with new configuration
5. **Phase 5**: Remove old keys once everything works

## Build Configuration (Future)

For production, you'll want to set environment variables at build time:

```bash
# Build with new API keys
SUPABASE_PUBLISHABLE_KEY=sb_publishable_... \
USE_NEW_API_KEYS=true \
trunk build
```

## Debug Utilities Added

- `test_supabase_connection()`: Test basic API connectivity  
- `debug_areas_request()`: Debug the specific areas endpoint
- `test_api_key_configs()`: Compare old vs new key performance

## Next Steps

1. Try the old JWT keys first to see if the 400 error persists
2. If old keys work, then the issue is with new key authentication
3. If old keys also fail, then the issue is with the query itself
4. Check Supabase dashboard for API key permissions and usage logs
