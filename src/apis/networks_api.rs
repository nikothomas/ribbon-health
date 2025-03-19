/*
 * ribbon-health
 *
 * An API for interacting with the data provided by Ribbon Health, including information about healthcare providers, locations, insurances, and more. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
use crate::apis::ContentType;

#[async_trait]
pub trait NetworksApi: Send + Sync {

    /// GET /network_analysis
    ///
    /// View a provider network across different geographies (i.e. counties).  #### Example Use Case In looking to expand to a new region, analyze existing provider networks in the region to understand how best to construct your own. 
    async fn get_network_analysis(&self,  params: GetNetworkAnalysisParams ) -> Result<ResponseContent<GetNetworkAnalysisSuccess>, Error<GetNetworkAnalysisError>>;
}

pub struct NetworksApiClient {
    configuration: Arc<configuration::Configuration>
}

impl NetworksApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}


/// struct for passing parameters to the method [`get_network_analysis`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetNetworkAnalysisParams {
    /// A unique identifier for a single provider network from the Insurances reference endpoint.
    pub insurance_id: String,
    /// SSA codes of the counties to run a network analysis on.   A maximum of 50 codes may be included in a single request.
    pub ssa_codes: String,
    /// When set to `true` the response will not list specific NPIs in the network but will continue to include the `npi_count` fields to let you know how many there are.  This parameter can be used to greater lower the amount of data sent back when specific NPIs are not necessary.
    pub exclude_npis: Option<bool>
}


#[async_trait]
impl NetworksApi for NetworksApiClient {
    /// View a provider network across different geographies (i.e. counties).  #### Example Use Case In looking to expand to a new region, analyze existing provider networks in the region to understand how best to construct your own. 
    async fn get_network_analysis(&self,  params: GetNetworkAnalysisParams ) -> Result<ResponseContent<GetNetworkAnalysisSuccess>, Error<GetNetworkAnalysisError>> {
        
        let GetNetworkAnalysisParams {
            insurance_id,
            ssa_codes,
            exclude_npis,
        } = params;
        

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/network_analysis", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("insurance_id", &insurance_id.to_string())]);
        local_var_req_builder = local_var_req_builder.query(&[("ssa_codes", &ssa_codes.to_string())]);
        if let Some(ref local_var_str) = exclude_npis {
            local_var_req_builder = local_var_req_builder.query(&[("exclude_npis", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetNetworkAnalysisSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetNetworkAnalysisError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed successes of method [`get_network_analysis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkAnalysisSuccess {
    Status200(models::GetNetworkAnalysis200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_network_analysis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkAnalysisError {
    Status400(models::GetCustomProviders400Response),
    UnknownValue(serde_json::Value),
}

