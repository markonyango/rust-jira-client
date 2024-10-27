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

/// JqlFunctionPrecomputationUpdateBean : Precomputation id and its new value.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationUpdateBean {
    /// The error message to be displayed to the user if the given function clause is no longer valid during recalculation of the precomputation.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The id of the precomputation to update.
    #[serde(rename = "id")]
    pub id: String,
    /// The new value of the precomputation.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl JqlFunctionPrecomputationUpdateBean {
    /// Precomputation id and its new value.
    pub fn new(id: String) -> JqlFunctionPrecomputationUpdateBean {
        JqlFunctionPrecomputationUpdateBean {
            error: None,
            id,
            value: None,
        }
    }
}

