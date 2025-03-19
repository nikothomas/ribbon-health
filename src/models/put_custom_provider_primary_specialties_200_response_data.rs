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
pub struct PutCustomProviderPrimarySpecialties200ResponseData {
    /// Whether or not this specialty is one of the provider's primary specialties.
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
}

impl PutCustomProviderPrimarySpecialties200ResponseData {
    pub fn new(is_primary: bool) -> PutCustomProviderPrimarySpecialties200ResponseData {
        PutCustomProviderPrimarySpecialties200ResponseData {
            is_primary,
        }
    }
}

