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

/// IssueFilterForBulkPropertyDelete : Bulk operation filter details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueFilterForBulkPropertyDelete {
    /// The value of properties to perform the bulk operation on.
    #[serde(rename = "currentValue", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<Option<serde_json::Value>>,
    /// List of issues to perform the bulk delete operation on.
    #[serde(rename = "entityIds", skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<i64>>,
}

impl IssueFilterForBulkPropertyDelete {
    /// Bulk operation filter details.
    pub fn new() -> IssueFilterForBulkPropertyDelete {
        IssueFilterForBulkPropertyDelete {
            current_value: None,
            entity_ids: None,
        }
    }
}

