mod pypxdb;
mod models;

use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/readyz", get(readyz))
        .at("/dicomweb/:name", get(hello));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

#[handler]
fn readyz() -> &'static str {
    "ok"
}
