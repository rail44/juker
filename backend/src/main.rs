use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::Form,
    Router,
};
use serde_json::json;
use serde::Deserialize;
use slack;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/status", get(status))
        .route("/command", post(command));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn status() -> &'static str {
    "ok"
}

#[derive(Deserialize)]
struct CommandRequest {
    trigger_id: String
}

async fn command(req: Form<CommandRequest>) -> impl IntoResponse {
    let token = env::var("SLACK_TOKEN").unwrap();
    let view = json!({
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Juker"
        },
        "blocks": [],
    })
    .to_string();
    let trigger_id = &req.trigger_id;
    println!("{trigger_id}");
    println!("{view}");

    slack::apis::views_api::views_open(
        &slack::apis::configuration::Configuration::default(),
        &token,
        trigger_id,
        &view,
    )
    .await
    .unwrap();

    StatusCode::OK
}
