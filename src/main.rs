mod cron;
mod handler;
mod middleware;
mod model;
mod server;
mod utils;
use std::sync::Arc;

use server::serve_api;

use model::{ExternApiConfig, InternalApiConfig};

#[tokio::main]
async fn main() {
    let ext_api = ExternApiConfig::get_ext_env();
    let int_api = InternalApiConfig::get_int_env();

    let conf_ext = Arc::new(ext_api);
    let conf_int = Arc::new(int_api);

    serve_api(conf_ext, conf_int).await;
}
