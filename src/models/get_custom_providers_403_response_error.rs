/*
 * ribbon-health
 *
 * An API for interacting with the data provided by Ribbon Health, including information about healthcare providers, locations, insurances, and more. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCustomProviders403ResponseError {
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "code")]
    pub code: Code,
    #[serde(rename = "message")]
    pub message: String,
}

impl GetCustomProviders403ResponseError {
    pub fn new(status: i32, code: Code, message: String) -> GetCustomProviders403ResponseError {
        GetCustomProviders403ResponseError {
            status,
            code,
            message,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "permission_denied")]
    PermissionDenied,
}

impl Default for Code {
    fn default() -> Code {
        Self::PermissionDenied
    }
}

