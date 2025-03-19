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
pub struct GetPricingProviders200ResponseDataInnerInsurance {
    /// The UUID of the insurance these prices correspond to.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    /// The name of the insurance plan this UUID represents.
    #[serde(rename = "plan_name", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// The carrier that this insurance plan is associated with, if any.
    #[serde(rename = "carrier_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<Option<String>>,
}

impl GetPricingProviders200ResponseDataInnerInsurance {
    pub fn new() -> GetPricingProviders200ResponseDataInnerInsurance {
        GetPricingProviders200ResponseDataInnerInsurance {
            uuid: None,
            plan_name: None,
            carrier_name: None,
        }
    }
}

