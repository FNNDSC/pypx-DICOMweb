#![allow(non_snake_case)]

mod models;
mod pypx_deserializer;
mod pypx_reader;
mod router;
mod translate;

use crate::router::PypxDicomWebRouter;
use poem::{get, handler, listener::TcpListener, Route, Server};
use poem_openapi::{ContactObject, OpenApiService};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = get_port();
    let base = get_base();
    let api = PypxDicomWebRouter::new(base).unwrap();
    let contact = ContactObject::new()
        .name("Jennings Zhang")
        .email("jennings.zhang@childrens.harvard.edu");
    let description = "DICOMweb implementation supporting query and retrieve from a \
    filesystem path containing DICOM instances organized by pypx.";

    let api_service = OpenApiService::new(api, "pypx DICOMweb", "0.1")
        .server(format!("http://localhost:{port}/dicomweb"))
        .description(description)
        .contact(contact);
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .at("/readyz", get(readyz))
        .nest("/", api_service)
        .nest("/docs", ui);

    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .run(app)
        .await
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

#[handler]
pub fn readyz() -> &'static str {
    "ok"
}
