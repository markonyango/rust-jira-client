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

/// CustomFieldOptionUpdate : Details of a custom field option for a context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldOptionUpdate {
    /// Whether the option is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The ID of the custom field option.
    #[serde(rename = "id")]
    pub id: String,
    /// The value of the custom field option.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CustomFieldOptionUpdate {
    /// Details of a custom field option for a context.
    pub fn new(id: String) -> CustomFieldOptionUpdate {
        CustomFieldOptionUpdate {
            disabled: None,
            id,
            value: None,
        }
    }
}
