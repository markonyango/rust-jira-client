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

/// BulkIssueIsWatching : A container for the watch status of a list of issues.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkIssueIsWatching {
    /// The map of issue ID to boolean watch status.
    #[serde(rename = "issuesIsWatching", skip_serializing_if = "Option::is_none")]
    pub issues_is_watching: Option<std::collections::HashMap<String, bool>>,
}

impl BulkIssueIsWatching {
    /// A container for the watch status of a list of issues.
    pub fn new() -> BulkIssueIsWatching {
        BulkIssueIsWatching {
            issues_is_watching: None,
        }
    }
}

