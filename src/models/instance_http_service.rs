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
pub struct InstanceHttpService {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "auth_mode", skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<models::HttpServiceAuthMode>,
}

impl InstanceHttpService {
    pub fn new(name: String, port: i32, url: String) -> InstanceHttpService {
        InstanceHttpService {
            name,
            port,
            url,
            auth_mode: None,
        }
    }
}

