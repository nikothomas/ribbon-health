use std::sync::Arc;
use dotenvy::dotenv;
use std::env;

use ribbon_health::apis::{configuration::Configuration, providers_api::{ProvidersApi, ProvidersApiClient, GetCustomProvidersParams}};

#[tokio::test]
async fn test_search_providers_san_diego() {
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
    let client = ProvidersApiClient::new(config);
    
    // Set up parameters for provider search
    let params = GetCustomProvidersParams {
        location: Some("32.7157,-117.1611".to_string()), // San Diego coordinates
        distance: Some(25.0),                            // Within 25 miles
        page_size: Some(50),                             // Get 50 results
        page: Some(1),                                   // First page
        // Explicitly set all required fields to None
        max_locations: None,
        fields: None,
        _excl_fields: None,
        npis: None,
        name: None,
        provider_types: None,
        _excl_provider_types: None,
        gender: None,
        max_age: None,
        min_age: None,
        language: None,
        _excl_language: None,
        min_rating: None,
        address: None,
        location_ids: None,
        _excl_location_ids: None,
        min_location_confidence: None,
        min_confidence: None,
        state: None,
        insurance_ids: None,
        _excl_insurance_ids: None,
        insurance_carrier_name: None,
        location_insurance_ids: None,
        _excl_location_insurance_ids: None,
        national_bluecard: None,
        specialty_ids: None,
        _excl_specialty_ids: None,
        specialty: None,
        specialty_ids_primary: None,
        _excl_specialty_ids_primary: None,
        specialty_primary: None,
        apply_specialty_grouping: None,
        procedure_ids: None,
        _excl_procedure_ids: None,
        procedure: None,
        min_experience_index: None,
        max_cost_index: None,
        clinical_area: None,
        clinical_area_ids: None,
        _excl_clinical_area_ids: None,
        condition: None,
        condition_ids: None,
        _excl_condition_ids: None,
        treatment: None,
        treatment_ids: None,
        _excl_treatment_ids: None,
        panel_ages: None,
        _excl_panel_ages: None,
        panel_sexes: None,
        min_outcomes_index: None,
        min_efficiency_index: None,
        max_unit_cost_index: None,
        max_ribbon_cost_score: None,
        location_organization_ids: None,
        _excl_location_organization_ids: None,
        tin_ids: None,
        tin_name: None,
        tin_legal_name: None,
    };
    
    // Execute the API call
    let result = client.get_custom_providers(params).await;
    
    // Verify the result
    match result {
        Ok(response) => {
            // Check if the response status is success
            assert!(response.status.is_success());
            
            // Check if we received provider data
            if let Some(provider_data) = response.entity {
                // The data is inside the GetCustomProvidersSuccess enum
                match provider_data {
                    ribbon_health::apis::providers_api::GetCustomProvidersSuccess::Status200(resp) => {
                        println!("Successfully parsed response as Status200");
                        println!("Found {} providers", resp.data.len());
                        assert!(!resp.data.is_empty(), "No providers found in the response");
                    },
                    ribbon_health::apis::providers_api::GetCustomProvidersSuccess::UnknownValue(value) => {
                        println!("Response parsed as UnknownValue - TEST FAILED");
                        if let Some(data) = value.get("data").and_then(|d| d.as_array()) {
                            println!("Found {} providers in UnknownValue", data.len());
                            panic!("Response was parsed as UnknownValue instead of Status200");
                        } else {
                            panic!("Response did not contain a data array");
                        }
                    }
                }
            } else {
                panic!("No provider data returned in successful response");
            }
        },
        Err(e) => {
            panic!("API request failed: {:?}", e);
        }
    }
} 