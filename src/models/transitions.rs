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

/// Transitions : List of issue transitions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transitions {
    /// Expand options that include additional transitions details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// List of issue transitions.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<models::IssueTransition>>,
}

impl Transitions {
    /// List of issue transitions.
    pub fn new() -> Transitions {
        Transitions {
            expand: None,
            transitions: None,
        }
    }
}

