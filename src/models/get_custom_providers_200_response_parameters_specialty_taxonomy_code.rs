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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomProviders200ResponseParametersSpecialtyTaxonomyCode {
    String(String),
}

impl Default for GetCustomProviders200ResponseParametersSpecialtyTaxonomyCode {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

