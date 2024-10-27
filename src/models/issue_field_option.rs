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

/// IssueFieldOption : Details of the options for a select list issue field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueFieldOption {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::IssueFieldOptionConfiguration>>,
    /// The unique identifier for the option. This is only unique within the select field's set of options.
    #[serde(rename = "id")]
    pub id: i64,
    /// The properties of the object, as arbitrary key-value pairs. These properties can be searched using JQL, if the extractions (see [Issue Field Option Property Index](https://developer.atlassian.com/cloud/jira/platform/modules/issue-field-option-property-index/)) are defined in the descriptor for the issue field module.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The option's name, which is displayed in Jira.
    #[serde(rename = "value")]
    pub value: String,
}

impl IssueFieldOption {
    /// Details of the options for a select list issue field.
    pub fn new(id: i64, value: String) -> IssueFieldOption {
        IssueFieldOption {
            config: None,
            id,
            properties: None,
            value,
        }
    }
}

