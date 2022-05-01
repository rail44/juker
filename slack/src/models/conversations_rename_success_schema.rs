/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsRenameSuccessSchema : Schema for successful response from conversations.rename method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationsRenameSuccessSchema {
    #[serde(rename = "channel")]
    pub channel: serde_json::Value,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ConversationsRenameSuccessSchema {
    /// Schema for successful response from conversations.rename method
    pub fn new(channel: serde_json::Value, ok: crate::models::DefsOkTrue) -> ConversationsRenameSuccessSchema {
        ConversationsRenameSuccessSchema {
            channel,
            ok,
        }
    }
}

