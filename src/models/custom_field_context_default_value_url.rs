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

/// CustomFieldContextDefaultValueUrl : The default value for a URL custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueUrl {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    /// The default URL.
    #[serde(rename = "url")]
    pub url: String,
}

impl CustomFieldContextDefaultValueUrl {
    /// The default value for a URL custom field.
    pub fn new(context_id: String, r#type: String, url: String) -> CustomFieldContextDefaultValueUrl {
        CustomFieldContextDefaultValueUrl {
            context_id,
            r#type,
            url,
        }
    }
}
