use std::sync::Arc;
use dotenvy::dotenv;
use std::env;

use ribbon_health::apis::{configuration::Configuration, reference_endpoints_api::{ReferenceEndpointsApi, ReferenceEndpointsApiClient, GetInsurancesParams}};

#[tokio::test]
async fn test_get_insurances() {
    // Load environment variables from .env file if available
    // Will continue if .env is not found, allowing use of system environment variables
    let _ = dotenv();
    
    // Get the bearer token from environment variable
    let bearer_token = env::var("RIBBON_API_TOKEN").expect("RIBBON_API_TOKEN must be set");
    
    // Create configuration with bearer token
    let mut config = Configuration::new();
    config.bearer_access_token = Some(bearer_token);
    let config = Arc::new(config);
    
    // Create API client
    let client = ReferenceEndpointsApiClient::new(config);
    
    // Set up parameters for insurance search
    let params = GetInsurancesParams {
        search: None,
        carrier_association: None,
        carrier_brand: None,
        carrier_name: Some("Aetna".to_string()),  // Search for Aetna insurance
        state: Some("ND".to_string()),
        plan_name: None,
        plan_type: None,
        display_name: None,
        category: None,
        _excl_category: None,
        codes: None,
        partial_codes: None,
    };
    
    // Execute the API call
    let result = client.get_insurances(params).await;
    
    // Verify the result
    match result {
        Ok(response) => {
            // Check if the response status is success
            assert!(response.status.is_success());
            
            // Check if we received insurance data
            if let Some(insurance_data) = response.entity {
                // The data is inside the GetInsurancesSuccess enum
                match insurance_data {
                    ribbon_health::apis::reference_endpoints_api::GetInsurancesSuccess::Status200(resp) => {
                        println!("Successfully parsed response as Status200");
                        println!("Found {} insurance plans", resp.count);
                        assert!(!resp.results.is_empty(), "No insurance plans found in the response");
                    },
                    ribbon_health::apis::reference_endpoints_api::GetInsurancesSuccess::UnknownValue(value) => {
                        println!("Response parsed as UnknownValue - TEST FAILED");
                        if let Some(results) = value.get("results").and_then(|r| r.as_array()) {
                            println!("Found {} insurance plans in UnknownValue", results.len());
                            panic!("Response was parsed as UnknownValue instead of Status200");
                        } else {
                            panic!("Response did not contain a results array");
                        }
                    }
                }
            } else {
                panic!("No insurance data returned in successful response");
            }
        },
        Err(e) => {
            panic!("API request failed: {:?}", e);
        }
    }
} 