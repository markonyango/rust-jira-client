/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-e098eec8c0925855876f3d946f20a4b01cd69e3c
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserPickerUser : A user found in a search.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPickerUser {
    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The avatar URL of the user.
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the user. Depending on the user’s privacy setting, this may be returned as null.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The display name, email address, and key of the user with the matched query string highlighted with the HTML bold tag.
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// This property is no longer available. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// This property is no longer available . See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UserPickerUser {
    /// A user found in a search.
    pub fn new() -> UserPickerUser {
        UserPickerUser {
            account_id: None,
            avatar_url: None,
            display_name: None,
            html: None,
            key: None,
            name: None,
        }
    }
}

