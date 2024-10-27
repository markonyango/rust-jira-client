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

/// WorkflowTrigger : The trigger configuration associated with a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    /// The ID of the trigger.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The parameters of the trigger.
    #[serde(rename = "parameters")]
    pub parameters: std::collections::HashMap<String, String>,
    /// The rule key of the trigger.
    #[serde(rename = "ruleKey")]
    pub rule_key: String,
}

impl WorkflowTrigger {
    /// The trigger configuration associated with a workflow.
    pub fn new(parameters: std::collections::HashMap<String, String>, rule_key: String) -> WorkflowTrigger {
        WorkflowTrigger {
            id: None,
            parameters,
            rule_key,
        }
    }
}
