use crate::env::{get_env, Env};
use reqwest::header::{CONTENT_TYPE, ACCEPT_CHARSET};
use serde::Deserialize;
use serde_json::json;
use std::env;
use tokio::sync::OnceCell;

static SLACK_TOKEN: OnceCell<String> = OnceCell::const_new();

async fn get_token() -> &'static str {
    SLACK_TOKEN
        .get_or_init(|| async { env::var("SLACK_TOKEN").unwrap_or_else(|_| "".to_string()) })
        .await
}

pub async fn post_message(body: &str) {
    if get_env().await == &Env::Dev {
        tracing::warn!("because of running in dev mode, skipping slac::post_message()");
        return;
    }

    let token = get_token().await;
    let client = reqwest::Client::new();
    let res = client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(token)
        .body(json!({
            "channel": "C03DXGTRP45",
            "text": body,
        }).to_string())
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .header(ACCEPT_CHARSET, "utf-8")
        .send()
        .await
        .unwrap();
    tracing::info!("{}", res.text().await.unwrap());
}

pub async fn view_open(trigger_id: &str) {
    if get_env().await == &Env::Dev {
        tracing::warn!("because of running in dev mode, skipping slac::view_open()");
        return;
    }

    let token = get_token().await;
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
                    "action_id": "input"
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
                    "action_id": "input"
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
    let res = client
        .post("https://slack.com/api/views.open")
        .bearer_auth(token)
        .body(json!({
            "trigger_id": trigger_id,
            "view": view,
        }).to_string())
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .header(ACCEPT_CHARSET, "utf-8")
        .send()
        .await
        .unwrap();
    tracing::info!("{}", res.text().await.unwrap());
}

#[derive(Deserialize)]
pub struct InteractiveRequest {
    pub payload: String,
}

#[derive(Deserialize)]
pub struct InteractiveUserPayload {
    pub username: String,
}

#[derive(Deserialize)]
pub struct InteractiveInputPayload<T> {
    pub input: InteractiveValuePayload<T>,
}

#[derive(Deserialize)]
pub struct InteractiveValuePayload<T> {
    pub value: T,
}

#[derive(Deserialize)]
pub struct InteractiveValuesPayload {
    pub url: InteractiveInputPayload<String>,
    pub like: InteractiveInputPayload<Option<String>>,
}

#[derive(Deserialize)]
pub struct InteractiveStatePayload {
    pub values: InteractiveValuesPayload,
}

#[derive(Deserialize)]
pub struct InteractiveViewPayload {
    pub state: InteractiveStatePayload,
}

#[derive(Deserialize)]
pub struct InteractivePayload {
    pub user: InteractiveUserPayload,
    pub view: InteractiveViewPayload,
}
