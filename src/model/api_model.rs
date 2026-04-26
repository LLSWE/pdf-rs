use std::sync::Arc;

use axum::extract::FromRef;

use crate::middleware::ApiKey;

#[derive(Clone)]
pub struct InternalApiConfig {
    pub port: String,
    pub api_key: String,
}

impl InternalApiConfig {
    pub fn get_int_env() -> Self {
        Self {
            port: std::env::var("PORT").unwrap_or("8169".to_string()),
            api_key: std::env::var("INTERNAL_API_KEY").expect("[ERROR] Missing api key"),
        }
    }
}

#[derive(Clone)]
pub struct ExternApiConfig {
    pub api_token_url: String,
    pub api_access_key: String,
}

impl ExternApiConfig {
    pub fn get_ext_env() -> Self {
        Self {
            api_token_url: std::env::var("API_TOKEN_URL").expect("[ERROR] Missing token url"),
            api_access_key: std::env::var("API_ACCESS_KEY").expect("[ERROR] Missing access key"),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub external: Arc<ExternApiConfig>,
    pub internal: Arc<InternalApiConfig>,
}

impl FromRef<AppState> for Arc<InternalApiConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.internal.clone()
    }
}

impl FromRef<AppState> for Arc<ExternApiConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.external.clone()
    }
}
