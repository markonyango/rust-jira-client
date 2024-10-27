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

/// ContextualConfiguration : Details of the contextual configuration for a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextualConfiguration {
    /// The field configuration.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<serde_json::Value>>,
    /// The ID of the field context the configuration is associated with.
    #[serde(rename = "fieldContextId")]
    pub field_context_id: String,
    /// The ID of the configuration.
    #[serde(rename = "id")]
    pub id: String,
    /// The field value schema.
    #[serde(rename = "schema", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Option<serde_json::Value>>,
}

impl ContextualConfiguration {
    /// Details of the contextual configuration for a custom field.
    pub fn new(field_context_id: String, id: String) -> ContextualConfiguration {
        ContextualConfiguration {
            configuration: None,
            field_context_id,
            id,
            schema: None,
        }
    }
}
