use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::Form,
    Router,
};
use serde_json::json;
use serde::Deserialize;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;

async fn slack_view_open(trigger_id: &str) {
    let token = env::var("SLACK_TOKEN").unwrap();
    let view = json!({
        "type": "modal",
        "title": {
            "type": "plain_text",
            "text": "Juker",
            "emoji": true
        },
        "callback_id": "identify_your_modals",
        "submit": {
            "type": "plain_text",
            "text": "Submit",
            "emoji": true
        },
        "close": {
            "type": "plain_text",
            "text": "Cancel",
            "emoji": true
        },
        "blocks": [
            {
                "type": "input",
                "block_id": "url",
                "element": {
                    "type": "plain_text_input",
                    "action_id": "text"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Youtube URL",
                    "emoji": true
                }
            },
            {
                "type": "input",
                "block_id": "like",
                "optional": true,
                "element": {
                    "type": "plain_text_input",
                    "action_id": "text"
                },
                "label": {
                    "type": "plain_text",
                    "text": "ここすき",
                    "emoji": true
                }
            }
        ]
    });
    let client = reqwest::Client::new();
    let res = client.post("https://slack.com/api/views.open")
        .bearer_auth(token.clone())
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .json(&json!({
            "token": token,
            "trigger_id": trigger_id,
            "view": view,
        }))
        .send()
        .await
        .unwrap();
    tracing::info!("{}", res.text().await.unwrap());
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/status", get(status))
        .route("/command", post(command))
        .route("/command", get(command))
        .route("/interactive", post(interactive));
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
    slack_view_open(&req.trigger_id).await;

    StatusCode::OK
}

#[derive(Deserialize)]
struct InteractiveRequest {
    payload: InteractivePayload
}

#[derive(Deserialize)]
struct InteractiveUserPayload {
    username: String
}

#[derive(Deserialize)]
struct InteractiveTextInputPayload {
    text: InteractiveTextValuePayload
}

#[derive(Deserialize)]
struct InteractiveTextValuePayload {
    value: String
}

#[derive(Deserialize)]
struct InteractiveValuesPayload {
    url: InteractiveTextInputPayload,
    like: InteractiveTextInputPayload,
}

#[derive(Deserialize)]
struct InteractiveStatePayload {
    values: InteractiveValuesPayload
}

#[derive(Deserialize)]
struct InteractivePayload {
    user: InteractiveUserPayload,
    state: InteractiveStatePayload,
}

async fn interactive(req: Form<InteractiveRequest>) -> impl IntoResponse {
    tracing::info!("url: {}", req.payload.state.values.url.text.value);
    tracing::info!("like: {}", req.payload.state.values.like.text.value);
    tracing::info!("username: {}", req.payload.user.username);
    StatusCode::OK
}
