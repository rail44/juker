use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod admin_api;
pub mod admin_apps_api;
pub mod admin_apps_approved_api;
pub mod admin_apps_requests_api;
pub mod admin_apps_restricted_api;
pub mod admin_conversations_api;
pub mod admin_conversations_ekm_api;
pub mod admin_conversations_restrict_access_api;
pub mod admin_emoji_api;
pub mod admin_invite_requests_api;
pub mod admin_invite_requests_approved_api;
pub mod admin_invite_requests_denied_api;
pub mod admin_teams_api;
pub mod admin_teams_admins_api;
pub mod admin_teams_owners_api;
pub mod admin_teams_settings_api;
pub mod admin_usergroups_api;
pub mod admin_users_api;
pub mod admin_users_session_api;
pub mod api_api;
pub mod apps_api;
pub mod apps_event_authorizations_api;
pub mod apps_permissions_api;
pub mod apps_permissions_resources_api;
pub mod apps_permissions_scopes_api;
pub mod apps_permissions_users_api;
pub mod auth_api;
pub mod bots_api;
pub mod calls_api;
pub mod calls_participants_api;
pub mod chat_api;
pub mod chat_scheduled_messages_api;
pub mod conversations_api;
pub mod dialog_api;
pub mod dnd_api;
pub mod emoji_api;
pub mod files_api;
pub mod files_comments_api;
pub mod files_remote_api;
pub mod migration_api;
pub mod oauth_api;
pub mod oauth_v2_api;
pub mod pins_api;
pub mod reactions_api;
pub mod reminders_api;
pub mod rtm_api;
pub mod search_api;
pub mod stars_api;
pub mod team_api;
pub mod team_profile_api;
pub mod usergroups_api;
pub mod usergroups_users_api;
pub mod users_api;
pub mod users_profile_api;
pub mod views_api;
pub mod workflows_api;

pub mod configuration;
