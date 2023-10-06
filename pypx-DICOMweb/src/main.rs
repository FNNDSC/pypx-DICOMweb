mod constants;
mod dicom;
mod errors;
mod json_files;
mod pypx_reader;
mod router;
mod translate;

use crate::pypx_reader::PypxReader;
use crate::router::get_router;
use axum::{http::Method, routing::get, Router};
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use axum_prometheus::PrometheusMetricLayerBuilder;

#[tokio::main]
async fn main() {
    init_logging();

    let port = get_port();
    let pypx = PypxReader::new(
        &get_path_env("PYPX_LOG_DIR"),
        get_path_env("PYPX_DATA_DIR"),
        get_path_env("PYPX_REPACK_DATA_MOUNTPOINT"),
    )
    .unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_prefix("pypx_dicomweb_axum")
        .with_ignore_pattern("/metrics")
        .with_default_metrics()
        .build_pair();
    let pypx_dicomweb_router = get_router(pypx)
        .layer(prometheus_layer);

    let app = Router::new()
        .route("/readyz", get(|| async { "OK" }))
        .nest("/dicomweb", pypx_dicomweb_router)
        .route("/metrics", get(|| async move { metric_handle.render()}))
        .layer(cors);

    let socket = format!("0.0.0.0:{port}").parse().unwrap();

    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_logging() {
    let log_format = std::env::var("RUST_LOG_FORMAT")
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|_| "pretty".to_string());
    if &log_format == "pretty" {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else if &log_format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else if ["", "no", "none"].contains(&&*log_format) {
    } else {
        panic!("Unsupported log format: {log_format}")
    }
}

fn get_port() -> u32 {
    let s = std::env::var("PORT").unwrap_or("4006".to_string());
    s.parse()
        .unwrap_or_else(|_| panic!("Failed to parse PORT={s} as an integer"))
}

fn get_path_env(name: &str) -> PathBuf {
    let s = std::env::var(name)
        .unwrap_or_else(|e| format!("Cannot read environment variable {name}: {e:?}"));
    PathBuf::from(s)
}
