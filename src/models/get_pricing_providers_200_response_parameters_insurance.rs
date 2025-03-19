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
pub struct GetPricingProviders200ResponseParametersInsurance {
    /// A UUID uniquely identifying this insurance
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    #[serde(rename = "plan_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<Option<String>>,
    #[serde(rename = "carrier_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<Option<String>>,
}

impl GetPricingProviders200ResponseParametersInsurance {
    pub fn new() -> GetPricingProviders200ResponseParametersInsurance {
        GetPricingProviders200ResponseParametersInsurance {
            uuid: None,
            plan_name: None,
            carrier_name: None,
        }
    }
}

