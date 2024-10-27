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

/// WorkflowStatus : Details of a workflow status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatus {
    /// The ID of the issue status.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the status in the workflow.
    #[serde(rename = "name")]
    pub name: String,
    /// Additional properties that modify the behavior of issues in this status. Supports the properties `jira.issue.editable` and `issueEditable` (deprecated) that indicate whether issues are editable.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl WorkflowStatus {
    /// Details of a workflow status.
    pub fn new(id: String, name: String) -> WorkflowStatus {
        WorkflowStatus {
            id,
            name,
            properties: None,
        }
    }
}

