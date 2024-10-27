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

/// UpdateDefaultProjectClassificationBean : The request for updating the default project classification level.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDefaultProjectClassificationBean {
    /// The ID of the project classification.
    #[serde(rename = "id")]
    pub id: String,
}

impl UpdateDefaultProjectClassificationBean {
    /// The request for updating the default project classification level.
    pub fn new(id: String) -> UpdateDefaultProjectClassificationBean {
        UpdateDefaultProjectClassificationBean {
            id,
        }
    }
}
