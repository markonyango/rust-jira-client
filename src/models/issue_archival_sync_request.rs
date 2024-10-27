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

/// IssueArchivalSyncRequest : List of Issue Ids Or Keys that are to be archived or unarchived
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueArchivalSyncRequest {
    #[serde(rename = "issueIdsOrKeys", skip_serializing_if = "Option::is_none")]
    pub issue_ids_or_keys: Option<Vec<String>>,
}

impl IssueArchivalSyncRequest {
    /// List of Issue Ids Or Keys that are to be archived or unarchived
    pub fn new() -> IssueArchivalSyncRequest {
        IssueArchivalSyncRequest {
            issue_ids_or_keys: None,
        }
    }
}

