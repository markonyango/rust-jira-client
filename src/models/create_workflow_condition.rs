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

/// CreateWorkflowCondition : A workflow transition condition.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowCondition {
    /// The list of workflow conditions.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::CreateWorkflowCondition>>,
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The compound condition operator.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// The type of the transition rule.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateWorkflowCondition {
    /// A workflow transition condition.
    pub fn new() -> CreateWorkflowCondition {
        CreateWorkflowCondition {
            conditions: None,
            configuration: None,
            operator: None,
            r#type: None,
        }
    }
}
/// The compound condition operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::And
    }
}

