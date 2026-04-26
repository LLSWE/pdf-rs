use crate::utils::get_token;
use axum::extract::State;
use std::sync::Arc;

use crate::model::ExternApiConfig;

pub async fn get_html_contracts(State(token_config): State<Arc<ExternApiConfig>>) {
    let token = get_token(&token_config).await;
}
