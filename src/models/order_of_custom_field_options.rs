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

/// OrderOfCustomFieldOptions : An ordered list of custom field option IDs and information on where to move them.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderOfCustomFieldOptions {
    /// The ID of the custom field option or cascading option to place the moved options after. Required if `position` isn't provided.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A list of IDs of custom field options to move. The order of the custom field option IDs in the list is the order they are given after the move. The list must contain custom field options or cascading options, but not both.
    #[serde(rename = "customFieldOptionIds")]
    pub custom_field_option_ids: Vec<String>,
    /// The position the custom field options should be moved to. Required if `after` isn't provided.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

impl OrderOfCustomFieldOptions {
    /// An ordered list of custom field option IDs and information on where to move them.
    pub fn new(custom_field_option_ids: Vec<String>) -> OrderOfCustomFieldOptions {
        OrderOfCustomFieldOptions {
            after: None,
            custom_field_option_ids,
            position: None,
        }
    }
}
/// The position the custom field options should be moved to. Required if `after` isn't provided.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Position {
    #[serde(rename = "First")]
    First,
    #[serde(rename = "Last")]
    Last,
}

impl Default for Position {
    fn default() -> Position {
        Self::First
    }
}

