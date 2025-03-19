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
pub struct GetEligibility200ResponseDeductibleDetailIndividual {
    #[serde(rename = "in_network", skip_serializing_if = "Option::is_none")]
    pub in_network: Option<models::GetEligibility200ResponseDeductibleDetailIndividualInNetwork>,
    #[serde(rename = "out_of_network", skip_serializing_if = "Option::is_none")]
    pub out_of_network: Option<models::GetEligibility200ResponseDeductibleDetailIndividualOutOfNetwork>,
}

impl GetEligibility200ResponseDeductibleDetailIndividual {
    pub fn new() -> GetEligibility200ResponseDeductibleDetailIndividual {
        GetEligibility200ResponseDeductibleDetailIndividual {
            in_network: None,
            out_of_network: None,
        }
    }
}

