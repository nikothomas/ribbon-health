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
pub struct GetEligibility200ResponseStatus {
    /// Whether the eligibility check was successful
    #[serde(rename = "is_valid", skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    /// If `is_valid` is `true`, an error code explaining what went wrong. When `is_valid` is `false`, this will be `null`.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<String>>,
}

impl GetEligibility200ResponseStatus {
    pub fn new() -> GetEligibility200ResponseStatus {
        GetEligibility200ResponseStatus {
            is_valid: None,
            error: None,
        }
    }
}

