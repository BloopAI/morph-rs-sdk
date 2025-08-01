/*
 * MorphVM API
 *
 * REST API for managing MorphVM instances and snapshots.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWakeRequest {
    /// Whether to wake the instance on HTTP request
    #[serde(rename = "wake_on_http", skip_serializing_if = "Option::is_none")]
    pub wake_on_http: Option<bool>,
    /// Whether to wake the instance on SSH request
    #[serde(rename = "wake_on_ssh", skip_serializing_if = "Option::is_none")]
    pub wake_on_ssh: Option<bool>,
}

impl UpdateWakeRequest {
    pub fn new() -> UpdateWakeRequest {
        UpdateWakeRequest {
            wake_on_http: None,
            wake_on_ssh: None,
        }
    }
}

