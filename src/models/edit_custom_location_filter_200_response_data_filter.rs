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
pub struct EditCustomLocationFilter200ResponseDataFilter {
    /// The name of filter which will be used when applying to a search
    #[serde(rename = "parameter")]
    pub parameter: String,
    /// The name of the field that the filter will be applied to  You can specify nested fields by placing a `.` between each level within the JSON object. For example, `address_details.state`
    #[serde(rename = "field")]
    pub field: String,
    /// The data type of the value passed into this filter
    #[serde(rename = "value_type")]
    pub value_type: ValueType,
    /// The type of comparison that will occur between the value passed into this filter and the field specified in the `field` parameter  Note that `boost` filters have several limitations: - A Boost Filter cannot use the following `value_type`s: `float` - If a Boost Filter's `field` targets something that is within a list of objects, such as ‘educations.education.name’, we will not reorder the list to bring these items to the front. We will merely boost records where there is an entry in the list that matches the filter, wherever it is.   - There is one exception to this: `locations`. If you search for a provider and boost fields nested within the `locations` list, we will reorder the locations to put matching locations first.
    #[serde(rename = "filter_type")]
    pub filter_type: FilterType,
}

impl EditCustomLocationFilter200ResponseDataFilter {
    pub fn new(parameter: String, field: String, value_type: ValueType, filter_type: FilterType) -> EditCustomLocationFilter200ResponseDataFilter {
        EditCustomLocationFilter200ResponseDataFilter {
            parameter,
            field,
            value_type,
            filter_type,
        }
    }
}
/// The data type of the value passed into this filter
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "list")]
    List,
}

impl Default for ValueType {
    fn default() -> ValueType {
        Self::String
    }
}
/// The type of comparison that will occur between the value passed into this filter and the field specified in the `field` parameter  Note that `boost` filters have several limitations: - A Boost Filter cannot use the following `value_type`s: `float` - If a Boost Filter's `field` targets something that is within a list of objects, such as ‘educations.education.name’, we will not reorder the list to bring these items to the front. We will merely boost records where there is an entry in the list that matches the filter, wherever it is.   - There is one exception to this: `locations`. If you search for a provider and boost fields nested within the `locations` list, we will reorder the locations to put matching locations first.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FilterType {
    #[serde(rename = "less_than")]
    LessThan,
    #[serde(rename = "greater_than")]
    GreaterThan,
    #[serde(rename = "equals")]
    Equals,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "boost")]
    Boost,
    #[serde(rename = "equals_any")]
    EqualsAny,
    #[serde(rename = "contains_any")]
    ContainsAny,
    #[serde(rename = "fuzzy")]
    Fuzzy,
}

impl Default for FilterType {
    fn default() -> FilterType {
        Self::LessThan
    }
}

