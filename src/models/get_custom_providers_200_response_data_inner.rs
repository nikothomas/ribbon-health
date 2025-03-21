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
pub struct GetCustomProviders200ResponseDataInner {
    /// The healthcare provider's 10-digit National Provider Identifier (NPI)
    #[serde(rename = "npi", skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    /// First name of the provider
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name of the provider
    #[serde(rename = "middle_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Last name of the provider
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The estimated age of the provider
    #[serde(rename = "age", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub age: Option<Option<i32>>,
    /// The gender of the provider
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// Total number of ratings collected across different sources
    #[serde(rename = "ratings_count", skip_serializing_if = "Option::is_none")]
    pub ratings_count: Option<i32>,
    /// Average patient satisfaction rating out of 10 points across multiple sources
    #[serde(rename = "ratings_avg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ratings_avg: Option<Option<f64>>,
    /// Lists all degrees associated with this provider (e.g. MD, OD, PhD)
    #[serde(rename = "degrees", skip_serializing_if = "Option::is_none")]
    pub degrees: Option<Vec<String>>,
    /// This lists all the specialties for a given provider
    #[serde(rename = "specialties", skip_serializing_if = "Option::is_none")]
    pub specialties: Option<Vec<models::GetCustomProviders200ResponseParametersSpecialty>>,
    /// List of confirmed languages spoken
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// List of the schools attended by the provider
    #[serde(rename = "educations", skip_serializing_if = "Option::is_none")]
    pub educations: Option<Vec<models::GetCustomProviders200ResponseDataInnerEducationsInner>>,
    /// List of insurances the provider accepts
    #[serde(rename = "insurances", skip_serializing_if = "Option::is_none")]
    pub insurances: Option<Vec<models::GetCustomProviders200ResponseDataInnerInsurancesInner>>,
    /// There are high level classifications for different provider types -- e.g. \"Doctor\", \"Optometry\", \"Dental Providers\", \"Nursing\", etc.
    #[serde(rename = "provider_types", skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<String>>,
    /// List of all locations this provider is known to practice at including any known phone numbers at these locations
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<models::GetCustomProviders200ResponseDataInnerLocationsInner>>,
    /// We aggregate profiles across a variety of different online sources, including booking platforms
    #[serde(rename = "online_profiles", skip_serializing_if = "Option::is_none")]
    pub online_profiles: Option<Vec<models::GetCustomProviders200ResponseDataInnerOnlineProfilesInner>>,
}

impl GetCustomProviders200ResponseDataInner {
    pub fn new() -> GetCustomProviders200ResponseDataInner {
        GetCustomProviders200ResponseDataInner {
            npi: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            age: None,
            gender: None,
            ratings_count: None,
            ratings_avg: None,
            degrees: None,
            specialties: None,
            languages: None,
            educations: None,
            insurances: None,
            provider_types: None,
            locations: None,
            online_profiles: None,
        }
    }
}
/// The gender of the provider
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "m")]
    M,
    #[serde(rename = "f")]
    F,
}

impl Default for Gender {
    fn default() -> Gender {
        Self::M
    }
}

