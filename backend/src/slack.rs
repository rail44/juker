use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_json::json;
use std::env;

pub async fn view_open(trigger_id: &str) {
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
    let res = client
        .post("https://slack.com/api/views.open")
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

#[derive(Deserialize)]
pub struct InteractiveRequest {
    pub payload: InteractivePayload,
}

#[derive(Deserialize)]
pub struct InteractiveUserPayload {
    pub username: String,
}

#[derive(Deserialize)]
pub struct InteractiveTextInputPayload {
    pub text: InteractiveTextValuePayload,
}

#[derive(Deserialize)]
pub struct InteractiveTextValuePayload {
    pub value: String,
}

#[derive(Deserialize)]
pub struct InteractiveValuesPayload {
    pub url: InteractiveTextInputPayload,
    pub like: InteractiveTextInputPayload,
}

#[derive(Deserialize)]
pub struct InteractiveStatePayload {
    pub values: InteractiveValuesPayload,
}

#[derive(Deserialize)]
pub struct InteractivePayload {
    pub user: InteractiveUserPayload,
    pub state: InteractiveStatePayload,
}
