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

/// IconBean : An icon.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconBean {
    /// The URL of the tooltip, used only for a status icon.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The title of the icon, for use as a tooltip on the icon.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of a 16x16 pixel icon.
    #[serde(rename = "url16x16", skip_serializing_if = "Option::is_none")]
    pub url16x16: Option<String>,
}

impl IconBean {
    /// An icon.
    pub fn new() -> IconBean {
        IconBean {
            link: None,
            title: None,
            url16x16: None,
        }
    }
}

