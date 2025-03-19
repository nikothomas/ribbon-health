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
pub struct GetPricingProviders200ResponseDataInnerProcedure {
    /// The UUID of the procedure these prices correspond to.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    /// The name of the procedure these prices correspond to.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetPricingProviders200ResponseDataInnerProcedure {
    pub fn new() -> GetPricingProviders200ResponseDataInnerProcedure {
        GetPricingProviders200ResponseDataInnerProcedure {
            uuid: None,
            name: None,
        }
    }
}

