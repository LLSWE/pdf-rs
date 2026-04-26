use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use futures::StreamExt;

use chromiumoxide::{Browser, BrowserConfig, cdp::browser_protocol::page::PrintToPdfParams};

use crate::middleware::ApiKey;
use crate::model::CreatePdf;

pub async fn generate_pdf(
    _auth: ApiKey,
    Json(html_file): Json<CreatePdf>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .build()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser
        .new_page(html_file.file_path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    page.wait_for_navigation()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let pdf_data = page
        .pdf(PrintToPdfParams::default())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tokio::fs::write(html_file.bucket_path, pdf_data)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("[ERRO] Erro ao salvar pdf : {}", e),
            )
        })?;

    browser.close().await.ok();

    handle
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}
