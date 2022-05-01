/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`team_access_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamAccessLogsError {
    DefaultResponse(crate::models::TeamAccessLogsErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`team_billable_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamBillableInfoError {
    DefaultResponse(crate::models::DefaultErrorTemplate),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`team_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamInfoError {
    DefaultResponse(crate::models::TeamInfoErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`team_integration_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamIntegrationLogsError {
    DefaultResponse(crate::models::TeamIntegrationLogsErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`team_profile_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamProfileGetError {
    DefaultResponse(crate::models::TeamProfileGetErrorSchema),
    UnknownValue(serde_json::Value),
}


/// Gets the access logs for the current team.
pub async fn team_access_logs(configuration: &configuration::Configuration, token: &str, before: Option<&str>, count: Option<&str>, page: Option<&str>) -> Result<crate::models::TeamAccessLogsSchema, Error<TeamAccessLogsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team.accessLogs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = before {
        local_var_req_builder = local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TeamAccessLogsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets billable users information for the current team.
pub async fn team_billable_info(configuration: &configuration::Configuration, token: &str, user: Option<&str>) -> Result<crate::models::DefaultSuccessTemplate, Error<TeamBillableInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team.billableInfo", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TeamBillableInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets information about the current team.
pub async fn team_info(configuration: &configuration::Configuration, token: &str, team: Option<&str>) -> Result<crate::models::TeamInfoSchema, Error<TeamInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team.info", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = team {
        local_var_req_builder = local_var_req_builder.query(&[("team", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TeamInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets the integration logs for the current team.
pub async fn team_integration_logs(configuration: &configuration::Configuration, token: &str, app_id: Option<&str>, change_type: Option<&str>, count: Option<&str>, page: Option<&str>, service_id: Option<&str>, user: Option<&str>) -> Result<crate::models::TeamIntegrationLogsSchema, Error<TeamIntegrationLogsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team.integrationLogs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = app_id {
        local_var_req_builder = local_var_req_builder.query(&[("app_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = change_type {
        local_var_req_builder = local_var_req_builder.query(&[("change_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = service_id {
        local_var_req_builder = local_var_req_builder.query(&[("service_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TeamIntegrationLogsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a team's profile.
pub async fn team_profile_get(configuration: &configuration::Configuration, token: &str, visibility: Option<&str>) -> Result<crate::models::TeamProfileGetSuccessSchema, Error<TeamProfileGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/team.profile.get", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = visibility {
        local_var_req_builder = local_var_req_builder.query(&[("visibility", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TeamProfileGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

