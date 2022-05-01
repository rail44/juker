use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/status", get(status))
        .route("/command", post(create_user));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn status() -> &'static str {
    "ok"
}

async fn create_user() -> impl IntoResponse {
    (StatusCode::CREATED, "")
}
