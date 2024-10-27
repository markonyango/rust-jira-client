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

/// License : Details about a license for the Jira instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct License {
    /// The applications under this license.
    #[serde(rename = "applications")]
    pub applications: Vec<models::LicensedApplication>,
}

impl License {
    /// Details about a license for the Jira instance.
    pub fn new(applications: Vec<models::LicensedApplication>) -> License {
        License {
            applications,
        }
    }
}

