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
pub struct GetPricingCarriers200Response {
    /// 
    #[serde(rename = "data")]
    pub data: Vec<models::GetPricingCarriers200ResponseDataInner>,
}

impl GetPricingCarriers200Response {
    pub fn new(data: Vec<models::GetPricingCarriers200ResponseDataInner>) -> GetPricingCarriers200Response {
        GetPricingCarriers200Response {
            data,
        }
    }
}

