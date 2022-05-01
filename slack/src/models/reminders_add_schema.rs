/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemindersAddSchema : Schema for successful response from reminders.add method



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemindersAddSchema {
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "reminder")]
    pub reminder: Box<crate::models::ObjsReminder>,
}

impl RemindersAddSchema {
    /// Schema for successful response from reminders.add method
    pub fn new(ok: crate::models::DefsOkTrue, reminder: crate::models::ObjsReminder) -> RemindersAddSchema {
        RemindersAddSchema {
            ok,
            reminder: Box::new(reminder),
        }
    }
}


