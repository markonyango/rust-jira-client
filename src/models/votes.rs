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

/// Votes : The details of votes on an issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Votes {
    /// Whether the user making this request has voted on the issue.
    #[serde(rename = "hasVoted", skip_serializing_if = "Option::is_none")]
    pub has_voted: Option<bool>,
    /// The URL of these issue vote details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// List of the users who have voted on this issue. An empty list is returned when the calling user doesn't have the *View voters and watchers* project permission.
    #[serde(rename = "voters", skip_serializing_if = "Option::is_none")]
    pub voters: Option<Vec<models::User>>,
    /// The number of votes on the issue.
    #[serde(rename = "votes", skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,
}

impl Votes {
    /// The details of votes on an issue.
    pub fn new() -> Votes {
        Votes {
            has_voted: None,
            param_self: None,
            voters: None,
            votes: None,
        }
    }
}

