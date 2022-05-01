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
pub struct AppsPermissionsInfoSchemaInfo {
    #[serde(rename = "app_home")]
    pub app_home: Box<crate::models::AppsPermissionsInfoSchemaInfoAppHome>,
    #[serde(rename = "channel")]
    pub channel: Box<crate::models::AppsPermissionsInfoSchemaInfoAppHome>,
    #[serde(rename = "group")]
    pub group: Box<crate::models::AppsPermissionsInfoSchemaInfoAppHome>,
    #[serde(rename = "im")]
    pub im: Box<crate::models::AppsPermissionsInfoSchemaInfoAppHome>,
    #[serde(rename = "mpim")]
    pub mpim: Box<crate::models::AppsPermissionsInfoSchemaInfoAppHome>,
    #[serde(rename = "team")]
    pub team: Box<crate::models::AppsPermissionsInfoSchemaInfoTeam>,
}

impl AppsPermissionsInfoSchemaInfo {
    pub fn new(app_home: crate::models::AppsPermissionsInfoSchemaInfoAppHome, channel: crate::models::AppsPermissionsInfoSchemaInfoAppHome, group: crate::models::AppsPermissionsInfoSchemaInfoAppHome, im: crate::models::AppsPermissionsInfoSchemaInfoAppHome, mpim: crate::models::AppsPermissionsInfoSchemaInfoAppHome, team: crate::models::AppsPermissionsInfoSchemaInfoTeam) -> AppsPermissionsInfoSchemaInfo {
        AppsPermissionsInfoSchemaInfo {
            app_home: Box::new(app_home),
            channel: Box::new(channel),
            group: Box::new(group),
            im: Box::new(im),
            mpim: Box::new(mpim),
            team: Box::new(team),
        }
    }
}


