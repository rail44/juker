/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsSetPurposeSuccessSchema : Schema for successful response from conversations.setPurpose method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationsSetPurposeSuccessSchema {
    #[serde(rename = "channel")]
    pub channel: serde_json::Value,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ConversationsSetPurposeSuccessSchema {
    /// Schema for successful response from conversations.setPurpose method
    pub fn new(channel: serde_json::Value, ok: crate::models::DefsOkTrue) -> ConversationsSetPurposeSuccessSchema {
        ConversationsSetPurposeSuccessSchema {
            channel,
            ok,
        }
    }
}

