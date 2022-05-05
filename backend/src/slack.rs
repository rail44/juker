use crate::env::{get_env, Env};
use reqwest::header::{ACCEPT_CHARSET, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;
use std::env;
use tokio::sync::OnceCell;

static SLACK_TOKEN: OnceCell<String> = OnceCell::const_new();

pub fn req_info_payload(
    title: &str,
    id: &str,
    user: &str,
    like: Option<&str>,
) -> Vec<serde_json::Value> {
    let mut blocks = vec![
        json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": format!(":tv: *{}*", title),
            }
        }),
        json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": format!(":bust_in_silhouette: *{}*", user),
            }
        }),
    ];

    if let Some(like) = like {
        blocks.push(json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": format!(":heart: *{}*", like),
            }
        }));
    }

    blocks.push(json!({
        "type": "actions",
        "elements": [
            {
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": ":eyes:",
                    "emoji": true
                },
                "url": "https://rail44.github.io/juker"
            },
            {
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": ":youtube:",
                    "emoji": true
                },
                "url": format!("https://www.youtube.com/watch?v={}", id)
            }
        ]
    }));
    blocks
}

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
        .body(
            json!({
                "channel": "C03DXGTRP45",
                "text": body,
            })
            .to_string(),
        )
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .header(ACCEPT_CHARSET, "utf-8")
        .send()
        .await
        .unwrap();
    tracing::info!("{}", res.text().await.unwrap());
}

pub async fn post_block_message(blocks: Vec<serde_json::Value>) {
    if get_env().await == &Env::Dev {
        tracing::warn!("because of running in dev mode, skipping slac::post_message()");
        return;
    }

    let token = get_token().await;
    let client = reqwest::Client::new();
    let res = client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(token)
        .body(
            json!({
                "channel": "C03DXGTRP45",
                "blocks": blocks,
            })
            .to_string(),
        )
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
        .body(
            json!({
                "trigger_id": trigger_id,
                "view": view,
            })
            .to_string(),
        )
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
#[serde(untagged)]
pub enum InteractivePayload {
    Submission(SubmissionPayload),
    Unknown(serde_json::Value),
}

#[derive(Deserialize)]
pub struct SubmissionPayload {
    pub user: InteractiveUserPayload,
    pub view: InteractiveViewPayload,
}
