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

/// AuditRecords : Container for a list of audit records.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditRecords {
    /// The requested or default limit on the number of audit items to be returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The number of audit items skipped before the first item in this list.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// The list of audit items.
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<models::AuditRecordBean>>,
    /// The total number of audit items returned.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl AuditRecords {
    /// Container for a list of audit records.
    pub fn new() -> AuditRecords {
        AuditRecords {
            limit: None,
            offset: None,
            records: None,
            total: None,
        }
    }
}
