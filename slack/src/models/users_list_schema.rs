/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UsersListSchema : Schema for successful response from users.list method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsersListSchema {
    #[serde(rename = "cache_ts")]
    pub cache_ts: i32,
    #[serde(rename = "members")]
    pub members: Vec<serde_json::Value>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata", skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<serde_json::Value>,
}

impl UsersListSchema {
    /// Schema for successful response from users.list method
    pub fn new(cache_ts: i32, members: Vec<serde_json::Value>, ok: crate::models::DefsOkTrue) -> UsersListSchema {
        UsersListSchema {
            cache_ts,
            members,
            ok,
            response_metadata: None,
        }
    }
}


