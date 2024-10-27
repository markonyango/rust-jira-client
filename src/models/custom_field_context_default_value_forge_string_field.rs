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

/// CustomFieldContextDefaultValueForgeStringField : The default text for a Forge string custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeStringField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default text. The maximum length is 254 characters.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueForgeStringField {
    /// The default text for a Forge string custom field.
    pub fn new(context_id: String, r#type: String) -> CustomFieldContextDefaultValueForgeStringField {
        CustomFieldContextDefaultValueForgeStringField {
            context_id,
            text: None,
            r#type,
        }
    }
}
