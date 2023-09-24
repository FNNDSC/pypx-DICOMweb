mod errors;
mod json_files;
mod pypx_reader;
mod router;
mod translate;

use crate::pypx_reader::PypxReader;
use crate::router::get_router;
use axum::{routing::get, Router};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let port = get_port();
    let pypx = PypxReader::new(get_base()).unwrap();

    let pypx_dicomweb_router = get_router(pypx);

    let app = Router::new()
        .route("/readyz", get(|| async { "OK" }))
        .nest("/dicomweb", pypx_dicomweb_router);

    let socket = format!("0.0.0.0:{port}").parse().unwrap();

    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_port() -> u32 {
    let s = std::env::var("PORT").unwrap_or("4006".to_string());
    s.parse()
        .expect(&format!("Failed to parse PORT={s} as an integer"))
}

fn get_base() -> PathBuf {
    let s =
        std::env::var("PYPX_BASE_PATH").expect("Environment variable PYPX_BASE_PATH must be set");
    PathBuf::from(s)
}
