use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::{Json, Form},
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
                "element": {
                    "type": "plain_text_input",
                    "action_id": "plain_text_input-acton"
                },
                "label": {
                    "type": "plain_text",
                    "text": "Youtube URL",
                    "emoji": true
                }
            },
            {
                "type": "input",
                "element": {
                    "type": "plain_text_input",
                    "action_id": "plain_text_input-action"
                },
                "label": {
                    "type": "plain_text",
                    "text": "ここすき",
                    "emoji": true
                }
            }
        ]
    })
    .to_string();
    let trigger_id = &req.trigger_id;

    println!("{trigger_id}");
    println!("{view}");
    let mut config = slack::apis::configuration::Configuration::default();
    config.oauth_access_token = Some(token.clone());

    if let Err(e) = slack::apis::views_api::views_open(
        &config,
        &token,
        trigger_id,
        &view,
    )
    .await {
        println!("{:?}", e);
    }


    StatusCode::OK
}

#[derive(Deserialize)]
struct InteractiveRequest {
}

async fn interactive(Json(req): Json<InteractiveRequest>) -> impl IntoResponse {
    StatusCode::OK
}
