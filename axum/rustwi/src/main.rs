use std::net::SocketAddr;

use axum::{body::Body, response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();
    let app: Router<Body> = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Should return implements IntoResponse
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
