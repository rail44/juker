/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsMembersSuccessSchema : Schema for successful response conversations.members method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationsMembersSuccessSchema {
    #[serde(rename = "members")]
    pub members: Vec<String>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata")]
    pub response_metadata: Box<crate::models::AdminConversationsGetTeamsSchemaResponseMetadata>,
}

impl ConversationsMembersSuccessSchema {
    /// Schema for successful response conversations.members method
    pub fn new(members: Vec<String>, ok: crate::models::DefsOkTrue, response_metadata: crate::models::AdminConversationsGetTeamsSchemaResponseMetadata) -> ConversationsMembersSuccessSchema {
        ConversationsMembersSuccessSchema {
            members,
            ok,
            response_metadata: Box::new(response_metadata),
        }
    }
}

