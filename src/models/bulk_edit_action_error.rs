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

/// BulkEditActionError : Errors of bulk edit action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEditActionError {
    /// The error messages.
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    /// The errors.
    #[serde(rename = "errors")]
    pub errors: std::collections::HashMap<String, String>,
}

impl BulkEditActionError {
    /// Errors of bulk edit action.
    pub fn new(error_messages: Vec<String>, errors: std::collections::HashMap<String, String>) -> BulkEditActionError {
        BulkEditActionError {
            error_messages,
            errors,
        }
    }
}

