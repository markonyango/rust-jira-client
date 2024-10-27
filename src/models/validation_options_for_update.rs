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

/// ValidationOptionsForUpdate : The level of validation to return from the API. If no values are provided, the default would return `WARNING` and `ERROR` level validation results.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationOptionsForUpdate {
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<Levels>>,
}

impl ValidationOptionsForUpdate {
    /// The level of validation to return from the API. If no values are provided, the default would return `WARNING` and `ERROR` level validation results.
    pub fn new() -> ValidationOptionsForUpdate {
        ValidationOptionsForUpdate {
            levels: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Levels {
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "ERROR")]
    Error,
}

impl Default for Levels {
    fn default() -> Levels {
        Self::Warning
    }
}

