use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};

use crate::model::InternalApiConfig;

pub struct ApiKey;

impl<S> FromRequestParts<S> for ApiKey
where
    Arc<InternalApiConfig>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = Arc::<InternalApiConfig>::from_ref(_state);

        let api_key_header = parts
            .headers
            .get("X-API-KEY")
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "[UNAUTHORIZED] Missing ApiKey\n"))?;

        if api_key_header == config.api_key {
            Ok(ApiKey)
        } else {
            Err((StatusCode::UNAUTHORIZED, "[UNAUTHORIZED] Invalid Api Key\n"))
        }
    }
}
