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

/// FilterDetails : Details of a filter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterDetails {
    /// \\[Experimental\\] Approximate last used time. Returns the date and time when the filter was last used. Returns `null` if the filter hasn't been used after tracking was enabled. For performance reasons, timestamps aren't updated in real time and therefore may not be exactly accurate.
    #[serde(rename = "approximateLastUsed", skip_serializing_if = "Option::is_none")]
    pub approximate_last_used: Option<String>,
    /// The description of the filter.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The groups and projects that can edit the filter. This can be specified when updating a filter, but not when creating a filter.
    #[serde(rename = "editPermissions", skip_serializing_if = "Option::is_none")]
    pub edit_permissions: Option<Vec<models::SharePermission>>,
    /// Expand options that include additional filter details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Whether the filter is selected as a favorite by any users, not including the filter owner.
    #[serde(rename = "favourite", skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    /// The count of how many users have selected this filter as a favorite, including the filter owner.
    #[serde(rename = "favouritedCount", skip_serializing_if = "Option::is_none")]
    pub favourited_count: Option<i64>,
    /// The unique identifier for the filter.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The JQL query for the filter. For example, *project = SSP AND issuetype = Bug*.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The name of the filter.
    #[serde(rename = "name")]
    pub name: String,
    /// The user who owns the filter. Defaults to the creator of the filter, however, Jira administrators can change the owner of a shared filter in the admin settings.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::User>>,
    /// A URL to view the filter results in Jira, using the [Search for issues using JQL](#api-rest-api-2-filter-search-get) operation with the filter's JQL string to return the filter results. For example, *https://your-domain.atlassian.net/rest/api/2/search?jql=project+%3D+SSP+AND+issuetype+%3D+Bug*.
    #[serde(rename = "searchUrl", skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    /// The URL of the filter.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The groups and projects that the filter is shared with. This can be specified when updating a filter, but not when creating a filter.
    #[serde(rename = "sharePermissions", skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<models::SharePermission>>,
    /// The users that are subscribed to the filter.
    #[serde(rename = "subscriptions", skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<models::FilterSubscription>>,
    /// A URL to view the filter results in Jira, using the ID of the filter. For example, *https://your-domain.atlassian.net/issues/?filter=10100*.
    #[serde(rename = "viewUrl", skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}

impl FilterDetails {
    /// Details of a filter.
    pub fn new(name: String) -> FilterDetails {
        FilterDetails {
            approximate_last_used: None,
            description: None,
            edit_permissions: None,
            expand: None,
            favourite: None,
            favourited_count: None,
            id: None,
            jql: None,
            name,
            owner: None,
            search_url: None,
            param_self: None,
            share_permissions: None,
            subscriptions: None,
            view_url: None,
        }
    }
}

