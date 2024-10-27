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

/// CreatePrioritySchemeDetails : Details of a new priority scheme
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePrioritySchemeDetails {
    /// The ID of the default priority for the priority scheme.
    #[serde(rename = "defaultPriorityId")]
    pub default_priority_id: i64,
    /// The description of the priority scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mappings of issue priorities for issues being migrated in and out of this priority scheme.
    #[serde(rename = "mappings", skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Box<models::PriorityMapping>>,
    /// The name of the priority scheme. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
    /// The IDs of priorities in the scheme.
    #[serde(rename = "priorityIds")]
    pub priority_ids: Vec<i64>,
    /// The IDs of projects that will use the priority scheme.
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
}

impl CreatePrioritySchemeDetails {
    /// Details of a new priority scheme
    pub fn new(default_priority_id: i64, name: String, priority_ids: Vec<i64>) -> CreatePrioritySchemeDetails {
        CreatePrioritySchemeDetails {
            default_priority_id,
            description: None,
            mappings: None,
            name,
            priority_ids,
            project_ids: None,
        }
    }
}

