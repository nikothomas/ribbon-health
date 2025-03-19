use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod cost_estimates_api;
pub mod filters_api;
pub mod focus_area_endpoints_api;
pub mod locations_api;
pub mod networks_api;
pub mod organizations_api;
pub mod price_transparency_api;
pub mod providers_api;
pub mod reference_endpoints_api;
pub mod tins_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn cost_estimates_api(&self) -> &dyn cost_estimates_api::CostEstimatesApi;
    fn filters_api(&self) -> &dyn filters_api::FiltersApi;
    fn focus_area_endpoints_api(&self) -> &dyn focus_area_endpoints_api::FocusAreaEndpointsApi;
    fn locations_api(&self) -> &dyn locations_api::LocationsApi;
    fn networks_api(&self) -> &dyn networks_api::NetworksApi;
    fn organizations_api(&self) -> &dyn organizations_api::OrganizationsApi;
    fn price_transparency_api(&self) -> &dyn price_transparency_api::PriceTransparencyApi;
    fn providers_api(&self) -> &dyn providers_api::ProvidersApi;
    fn reference_endpoints_api(&self) -> &dyn reference_endpoints_api::ReferenceEndpointsApi;
    fn tins_api(&self) -> &dyn tins_api::TinsApi;
}

pub struct ApiClient {
    cost_estimates_api: Box<dyn cost_estimates_api::CostEstimatesApi>,
    filters_api: Box<dyn filters_api::FiltersApi>,
    focus_area_endpoints_api: Box<dyn focus_area_endpoints_api::FocusAreaEndpointsApi>,
    locations_api: Box<dyn locations_api::LocationsApi>,
    networks_api: Box<dyn networks_api::NetworksApi>,
    organizations_api: Box<dyn organizations_api::OrganizationsApi>,
    price_transparency_api: Box<dyn price_transparency_api::PriceTransparencyApi>,
    providers_api: Box<dyn providers_api::ProvidersApi>,
    reference_endpoints_api: Box<dyn reference_endpoints_api::ReferenceEndpointsApi>,
    tins_api: Box<dyn tins_api::TinsApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            cost_estimates_api: Box::new(cost_estimates_api::CostEstimatesApiClient::new(configuration.clone())),
            filters_api: Box::new(filters_api::FiltersApiClient::new(configuration.clone())),
            focus_area_endpoints_api: Box::new(focus_area_endpoints_api::FocusAreaEndpointsApiClient::new(configuration.clone())),
            locations_api: Box::new(locations_api::LocationsApiClient::new(configuration.clone())),
            networks_api: Box::new(networks_api::NetworksApiClient::new(configuration.clone())),
            organizations_api: Box::new(organizations_api::OrganizationsApiClient::new(configuration.clone())),
            price_transparency_api: Box::new(price_transparency_api::PriceTransparencyApiClient::new(configuration.clone())),
            providers_api: Box::new(providers_api::ProvidersApiClient::new(configuration.clone())),
            reference_endpoints_api: Box::new(reference_endpoints_api::ReferenceEndpointsApiClient::new(configuration.clone())),
            tins_api: Box::new(tins_api::TinsApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn cost_estimates_api(&self) -> &dyn cost_estimates_api::CostEstimatesApi {
        self.cost_estimates_api.as_ref()
    }
    fn filters_api(&self) -> &dyn filters_api::FiltersApi {
        self.filters_api.as_ref()
    }
    fn focus_area_endpoints_api(&self) -> &dyn focus_area_endpoints_api::FocusAreaEndpointsApi {
        self.focus_area_endpoints_api.as_ref()
    }
    fn locations_api(&self) -> &dyn locations_api::LocationsApi {
        self.locations_api.as_ref()
    }
    fn networks_api(&self) -> &dyn networks_api::NetworksApi {
        self.networks_api.as_ref()
    }
    fn organizations_api(&self) -> &dyn organizations_api::OrganizationsApi {
        self.organizations_api.as_ref()
    }
    fn price_transparency_api(&self) -> &dyn price_transparency_api::PriceTransparencyApi {
        self.price_transparency_api.as_ref()
    }
    fn providers_api(&self) -> &dyn providers_api::ProvidersApi {
        self.providers_api.as_ref()
    }
    fn reference_endpoints_api(&self) -> &dyn reference_endpoints_api::ReferenceEndpointsApi {
        self.reference_endpoints_api.as_ref()
    }
    fn tins_api(&self) -> &dyn tins_api::TinsApi {
        self.tins_api.as_ref()
    }
}


