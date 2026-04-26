use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::{
    handler::{generate_pdf, get_html_contracts, health},
    model::{AppState, ExternApiConfig, InternalApiConfig},
};

pub async fn serve_api(
    ext_api_config: Arc<ExternApiConfig>,
    int_api_config: Arc<InternalApiConfig>,
) {
    let port = &int_api_config.port;
    let addr = format!("0.0.0.0:{port}");

    let state = AppState {
        external: ext_api_config,
        internal: int_api_config,
    };

    let app = Router::new()
        .route("/", get(health))
        .route("/pdf/html-contract", get(get_html_contracts))
        .route("/pdf/generate", post(generate_pdf))
        .with_state(state);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("ERROR: failed to bind port");

    println!("Server is running on http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("ERROR: failed to serve the api");
}
