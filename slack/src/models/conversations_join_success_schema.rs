/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsJoinSuccessSchema : Schema for successful response from conversations.join method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationsJoinSuccessSchema {
    #[serde(rename = "channel")]
    pub channel: serde_json::Value,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata", skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<Box<crate::models::ResponseMetadata>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

impl ConversationsJoinSuccessSchema {
    /// Schema for successful response from conversations.join method
    pub fn new(channel: serde_json::Value, ok: crate::models::DefsOkTrue) -> ConversationsJoinSuccessSchema {
        ConversationsJoinSuccessSchema {
            channel,
            ok,
            response_metadata: None,
            warning: None,
        }
    }
}


