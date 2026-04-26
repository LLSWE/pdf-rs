use reqwest::StatusCode;

use crate::model::ExternApiConfig;

pub async fn get_token(api_creds: &ExternApiConfig) -> Result<String, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let resp = client
        .get(&api_creds.api_token_url)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let body = resp
        .text()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(body)
}
