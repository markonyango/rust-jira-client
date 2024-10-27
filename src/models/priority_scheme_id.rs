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

/// PrioritySchemeId : The ID of a priority scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrioritySchemeId {
    /// The ID of the priority scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The in-progress issue migration task.
    #[serde(rename = "task", skip_serializing_if = "Option::is_none")]
    pub task: Option<Box<models::TaskProgressBeanJsonNode>>,
}

impl PrioritySchemeId {
    /// The ID of a priority scheme.
    pub fn new() -> PrioritySchemeId {
        PrioritySchemeId {
            id: None,
            task: None,
        }
    }
}

