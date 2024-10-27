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

/// ValueOperand : An operand that is a user-provided value.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueOperand {
    /// Encoded value, which can be used directly in a JQL query.
    #[serde(rename = "encodedValue", skip_serializing_if = "Option::is_none")]
    pub encoded_value: Option<String>,
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
}

impl ValueOperand {
    /// An operand that is a user-provided value.
    pub fn new(value: String) -> ValueOperand {
        ValueOperand {
            encoded_value: None,
            value,
        }
    }
}

