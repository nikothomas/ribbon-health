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
pub struct GetEligibility200ResponseOutOfPocketDetail {
    #[serde(rename = "individual", skip_serializing_if = "Option::is_none")]
    pub individual: Option<models::GetEligibility200ResponseOutOfPocketDetailIndividual>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<models::GetEligibility200ResponseOutOfPocketDetailIndividual>,
}

impl GetEligibility200ResponseOutOfPocketDetail {
    pub fn new() -> GetEligibility200ResponseOutOfPocketDetail {
        GetEligibility200ResponseOutOfPocketDetail {
            individual: None,
            family: None,
        }
    }
}

