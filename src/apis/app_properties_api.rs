/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-e098eec8c0925855876f3d946f20a4b01cd69e3c
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`addon_properties_resource_period_delete_addon_property_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodDeleteAddonPropertyDeleteError {
    Status400(models::OperationMessage),
    Status401(models::OperationMessage),
    Status404(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`addon_properties_resource_period_get_addon_properties_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodGetAddonPropertiesGetError {
    Status401(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`addon_properties_resource_period_get_addon_property_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodGetAddonPropertyGetError {
    Status400(models::OperationMessage),
    Status401(models::OperationMessage),
    Status404(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`addon_properties_resource_period_put_addon_property_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodPutAddonPropertyPutError {
    Status400(models::OperationMessage),
    Status401(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_forge_app_property`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteForgeAppPropertyError {
    Status400(models::OperationMessage),
    Status401(),
    Status403(),
    Status404(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_forge_app_property`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutForgeAppPropertyError {
    Status400(models::OperationMessage),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}


/// Deletes an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
pub async fn addon_properties_resource_period_delete_addon_property_delete(configuration: &configuration::Configuration, addon_key: &str, property_key: &str) -> Result<(), Error<AddonPropertiesResourcePeriodDeleteAddonPropertyDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}", local_var_configuration.base_path, addonKey=crate::apis::urlencode(addon_key), propertyKey=crate::apis::urlencode(property_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AddonPropertiesResourcePeriodDeleteAddonPropertyDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets all the properties of an app.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
pub async fn addon_properties_resource_period_get_addon_properties_get(configuration: &configuration::Configuration, addon_key: &str) -> Result<models::PropertyKeys, Error<AddonPropertiesResourcePeriodGetAddonPropertiesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties", local_var_configuration.base_path, addonKey=crate::apis::urlencode(addon_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddonPropertiesResourcePeriodGetAddonPropertiesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the key and value of an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
pub async fn addon_properties_resource_period_get_addon_property_get(configuration: &configuration::Configuration, addon_key: &str, property_key: &str) -> Result<models::EntityProperty, Error<AddonPropertiesResourcePeriodGetAddonPropertyGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}", local_var_configuration.base_path, addonKey=crate::apis::urlencode(addon_key), propertyKey=crate::apis::urlencode(property_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddonPropertiesResourcePeriodGetAddonPropertyGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the value of an app's property. Use this resource to store custom data for your app.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
pub async fn addon_properties_resource_period_put_addon_property_put(configuration: &configuration::Configuration, addon_key: &str, property_key: &str, body: Option<serde_json::Value>) -> Result<models::OperationMessage, Error<AddonPropertiesResourcePeriodPutAddonPropertyPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}", local_var_configuration.base_path, addonKey=crate::apis::urlencode(addon_key), propertyKey=crate::apis::urlencode(property_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddonPropertiesResourcePeriodPutAddonPropertyPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a Forge app's property.  **[Permissions](#permissions) required:** Only Forge apps can make this request.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
pub async fn delete_forge_app_property(configuration: &configuration::Configuration, property_key: &str) -> Result<(), Error<DeleteForgeAppPropertyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/forge/1/app/properties/{propertyKey}", local_var_configuration.base_path, propertyKey=crate::apis::urlencode(property_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteForgeAppPropertyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the value of a Forge app's property. These values can be retrieved in [Jira expressions](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/) through the `app` [context variable](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#context-variables).  For other use cases, use the [Storage API](https://developer.atlassian.com/platform/forge/runtime-reference/storage-api/).  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only Forge apps can make this request.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
pub async fn put_forge_app_property(configuration: &configuration::Configuration, property_key: &str, body: Option<serde_json::Value>) -> Result<models::OperationMessage, Error<PutForgeAppPropertyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/forge/1/app/properties/{propertyKey}", local_var_configuration.base_path, propertyKey=crate::apis::urlencode(property_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutForgeAppPropertyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

