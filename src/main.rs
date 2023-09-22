mod pypx_reader;
mod pypx_deserializer;
mod translate;
mod router;
mod models;

use std::path::PathBuf;
use poem::{get, handler, listener::TcpListener, IntoResponse, Route, Server};
use poem_openapi::OpenApiService;
use crate::router::PypxDicomWebRouter;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 4006;
    let base = PathBuf::from("/home/jenni/fnndsc/pypx-listener/examples/px-repack-output");
    let api = PypxDicomWebRouter::new(base).unwrap();
    let api_service =
        OpenApiService::new(api, "pypx DICOMweb", "0.1").server(format!("http://localhost:{port}/dicomweb"));
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .at("/readyz", get(readyz))
        .nest("/dicomweb", api_service)
        .nest("/docs", ui);

    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .run(app)
        .await
}

#[handler]
pub fn readyz() -> &'static str {
    "ok"
}
