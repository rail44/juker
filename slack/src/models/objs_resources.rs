/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjsResources {
    #[serde(rename = "excluded_ids", skip_serializing_if = "Option::is_none")]
    pub excluded_ids: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ids")]
    pub ids: Vec<serde_json::Value>,
    #[serde(rename = "wildcard", skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<bool>,
}

impl ObjsResources {
    pub fn new(ids: Vec<serde_json::Value>) -> ObjsResources {
        ObjsResources {
            excluded_ids: None,
            ids,
            wildcard: None,
        }
    }
}


