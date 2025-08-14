/// Configuration for Supabase connection supporting both old JWT-based keys and new API key system
pub struct SupabaseConfig {
    pub url: String,
    pub api_key: String,
    pub use_new_keys: bool,
}

impl SupabaseConfig {
    pub fn new() -> Self {
        // Temporarily use old JWT keys to test if 400 error is related to new API keys
        // TODO: Switch back to with_new_keys() once we confirm the issue
        Self::with_old_keys()
    }
    
    /// Create config using the new API key system with your specific keys
    pub fn with_new_keys() -> Self {
        Self {
            url: "https://kwedqsqfgiydxypbfntb.supabase.co".to_string(),
            api_key: "sb_publishable_QodDkQP086yqqc_BmF3UPQ_YkJpNy4a".to_string(),
            use_new_keys: true,
        }
    }
    
    /// Create config using the old JWT-based keys
    pub fn with_old_keys() -> Self {
        Self {
            url: "https://kwedqsqfgiydxypbfntb.supabase.co".to_string(),
            api_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imt3ZWRxc3FmZ2l5ZHh5cGJmbnRiIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDU4NTM0MDcsImV4cCI6MjA2MTQyOTQwN30._MkW4-EcPk85UiBWM3HdHmeEeqYkffWDhSlaRmnS-XY".to_string(),
            use_new_keys: false,
        }
    }
    
    /// Create config with specific keys
    pub fn with_keys(publishable_key: Option<&str>, secret_key: Option<&str>, use_new_keys: bool) -> Self {
        let url = option_env!("SUPABASE_URL")
            .unwrap_or("https://kwedqsqfgiydxypbfntb.supabase.co");
        
        let api_key = if use_new_keys {
            // Use publishable key for client-side operations (preferred for WASM)
            publishable_key
                .or_else(|| option_env!("SUPABASE_PUBLISHABLE_KEY"))
                .unwrap_or_else(|| {
                    // Fallback to old anon key if no publishable key provided
                    option_env!("SUPABASE_ANON_KEY")
                        .unwrap_or("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imt3ZWRxc3FmZ2l5ZHh5cGJmbnRiIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDU4NTM0MDcsImV4cCI6MjA2MTQyOTQwN30._MkW4-EcPk85UiBWM3HdHmeEeqYkffWDhSlaRmnS-XY")
                })
        } else {
            // Use the old JWT-based anon key
            option_env!("SUPABASE_ANON_KEY")
                .unwrap_or("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Imt3ZWRxc3FmZ2l5ZHh5cGJmbnRiIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDU4NTM0MDcsImV4cCI6MjA2MTQyOTQwN30._MkW4-EcPk85UiBWM3HdHmeEeqYkffWDhSlaRmnS-XY")
        };

        Self {
            url: url.to_string(),
            api_key: api_key.to_string(),
            use_new_keys,
        }
    }
    
    /// Check if the current key is a JWT (starts with 'eyJ')
    pub fn is_jwt_key(&self) -> bool {
        self.api_key.starts_with("eyJ")
    }
    
    /// Check if we should include Authorization Bearer header
    pub fn needs_auth_header(&self) -> bool {
        // Always include Authorization header for JWT keys
        // For new API keys, we only include apikey header
        self.is_jwt_key()
    }
}

impl Default for SupabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}
