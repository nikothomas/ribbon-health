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
pub struct GetPricingProviderProcedureLocation200ResponseData {
    #[serde(rename = "costs", skip_serializing_if = "Option::is_none")]
    pub costs: Option<models::GetPricingProviderProcedure200ResponseLocationsInnerCosts>,
}

impl GetPricingProviderProcedureLocation200ResponseData {
    pub fn new() -> GetPricingProviderProcedureLocation200ResponseData {
        GetPricingProviderProcedureLocation200ResponseData {
            costs: None,
        }
    }
}

