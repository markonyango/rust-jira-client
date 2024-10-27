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

/// RemoteIssueLink : Details of an issue remote link.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteIssueLink {
    /// Details of the remote application the linked item is in.
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<models::Application>,
    /// The global ID of the link, such as the ID of the item on the remote system.
    #[serde(rename = "globalId", skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    /// The ID of the link.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Details of the item linked to.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<models::RemoteObject>,
    /// Description of the relationship between the issue and the linked item.
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// The URL of the link.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl RemoteIssueLink {
    /// Details of an issue remote link.
    pub fn new() -> RemoteIssueLink {
        RemoteIssueLink {
            application: None,
            global_id: None,
            id: None,
            object: None,
            relationship: None,
            param_self: None,
        }
    }
}

