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

/// SuggestedMappingsForPrioritiesRequestBean : Details of changes to a priority scheme's priorities that require suggested priority mappings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedMappingsForPrioritiesRequestBean {
    /// The ids of priorities being removed from the scheme.
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<i64>>,
    /// The ids of priorities being removed from the scheme.
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<i64>>,
}

impl SuggestedMappingsForPrioritiesRequestBean {
    /// Details of changes to a priority scheme's priorities that require suggested priority mappings.
    pub fn new() -> SuggestedMappingsForPrioritiesRequestBean {
        SuggestedMappingsForPrioritiesRequestBean {
            add: None,
            remove: None,
        }
    }
}

