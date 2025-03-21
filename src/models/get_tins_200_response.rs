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
pub struct GetTins200Response {
    #[serde(rename = "parameters")]
    pub parameters: models::GetTins200ResponseParameters,
    /// array of returned TIN objects
    #[serde(rename = "tins")]
    pub tins: Vec<models::GetTins200ResponseTinsInner>,
}

impl GetTins200Response {
    pub fn new(parameters: models::GetTins200ResponseParameters, tins: Vec<models::GetTins200ResponseTinsInner>) -> GetTins200Response {
        GetTins200Response {
            parameters,
            tins,
        }
    }
}

