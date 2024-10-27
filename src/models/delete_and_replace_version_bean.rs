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
pub struct DeleteAndReplaceVersionBean {
    /// An array of custom field IDs (`customFieldId`) and version IDs (`moveTo`) to update when the fields contain the deleted version.
    #[serde(rename = "customFieldReplacementList", skip_serializing_if = "Option::is_none")]
    pub custom_field_replacement_list: Option<Vec<models::CustomFieldReplacement>>,
    /// The ID of the version to update `affectedVersion` to when the field contains the deleted version.
    #[serde(rename = "moveAffectedIssuesTo", skip_serializing_if = "Option::is_none")]
    pub move_affected_issues_to: Option<i64>,
    /// The ID of the version to update `fixVersion` to when the field contains the deleted version.
    #[serde(rename = "moveFixIssuesTo", skip_serializing_if = "Option::is_none")]
    pub move_fix_issues_to: Option<i64>,
}

impl DeleteAndReplaceVersionBean {
    pub fn new() -> DeleteAndReplaceVersionBean {
        DeleteAndReplaceVersionBean {
            custom_field_replacement_list: None,
            move_affected_issues_to: None,
            move_fix_issues_to: None,
        }
    }
}

