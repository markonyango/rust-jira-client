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

/// DashboardDetails : Details of a dashboard.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardDetails {
    /// The description of the dashboard.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The edit permissions for the dashboard.
    #[serde(rename = "editPermissions")]
    pub edit_permissions: Vec<models::SharePermission>,
    /// The name of the dashboard.
    #[serde(rename = "name")]
    pub name: String,
    /// The share permissions for the dashboard.
    #[serde(rename = "sharePermissions")]
    pub share_permissions: Vec<models::SharePermission>,
}

impl DashboardDetails {
    /// Details of a dashboard.
    pub fn new(edit_permissions: Vec<models::SharePermission>, name: String, share_permissions: Vec<models::SharePermission>) -> DashboardDetails {
        DashboardDetails {
            description: None,
            edit_permissions,
            name,
            share_permissions,
        }
    }
}

