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

/// ScreenDetails : Details of a screen.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenDetails {
    /// The description of the screen. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
}

impl ScreenDetails {
    /// Details of a screen.
    pub fn new(name: String) -> ScreenDetails {
        ScreenDetails {
            description: None,
            name,
        }
    }
}
