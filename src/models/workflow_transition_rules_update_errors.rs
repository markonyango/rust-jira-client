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

/// WorkflowTransitionRulesUpdateErrors : Details of any errors encountered while updating workflow transition rules.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRulesUpdateErrors {
    /// A list of workflows.
    #[serde(rename = "updateResults")]
    pub update_results: Vec<models::WorkflowTransitionRulesUpdateErrorDetails>,
}

impl WorkflowTransitionRulesUpdateErrors {
    /// Details of any errors encountered while updating workflow transition rules.
    pub fn new(update_results: Vec<models::WorkflowTransitionRulesUpdateErrorDetails>) -> WorkflowTransitionRulesUpdateErrors {
        WorkflowTransitionRulesUpdateErrors {
            update_results,
        }
    }
}

