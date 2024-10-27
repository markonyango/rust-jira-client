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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveOptionFromIssuesResult {
    /// A collection of errors related to unchanged issues. The collection size is limited, which means not all errors may be returned.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<models::SimpleErrorCollection>>,
    /// The IDs of the modified issues.
    #[serde(rename = "modifiedIssues", skip_serializing_if = "Option::is_none")]
    pub modified_issues: Option<Vec<i64>>,
    /// The IDs of the unchanged issues, those issues where errors prevent modification.
    #[serde(rename = "unmodifiedIssues", skip_serializing_if = "Option::is_none")]
    pub unmodified_issues: Option<Vec<i64>>,
}

impl RemoveOptionFromIssuesResult {
    pub fn new() -> RemoveOptionFromIssuesResult {
        RemoveOptionFromIssuesResult {
            errors: None,
            modified_issues: None,
            unmodified_issues: None,
        }
    }
}

