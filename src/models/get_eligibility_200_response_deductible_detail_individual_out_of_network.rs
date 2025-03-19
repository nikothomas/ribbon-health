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
pub struct GetEligibility200ResponseDeductibleDetailIndividualOutOfNetwork {
    /// The amount the member has paid towards out-of-network deductibles, in dollars
    #[serde(rename = "spend", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub spend: Option<Option<String>>,
    /// The maximum amount the member can pay towards out-of-network deductibles, in dollars
    #[serde(rename = "maximum", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Option<String>>,
    /// The remaining amount the member can pay towards out-of-network deductibles, in dollars
    #[serde(rename = "remaining", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remaining: Option<Option<String>>,
}

impl GetEligibility200ResponseDeductibleDetailIndividualOutOfNetwork {
    pub fn new() -> GetEligibility200ResponseDeductibleDetailIndividualOutOfNetwork {
        GetEligibility200ResponseDeductibleDetailIndividualOutOfNetwork {
            spend: None,
            maximum: None,
            remaining: None,
        }
    }
}

