# Rust API client for ribbon-health

An API client for interacting with the data provided by Ribbon Health, including information about healthcare providers, locations, insurances, and more.

[![Crates.io Version](https://img.shields.io/crates/v/ribbon-health.svg)](https://crates.io/crates/ribbon-health)
[![Tests Status](https://github.com/nikothomas/ribbon-health/actions/workflows/test.yml/badge.svg)](https://github.com/nikothomas/ribbon-health/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/nikothomas/ribbon-health/branch/main/graph/badge.svg)](https://codecov.io/gh/nikothomas/ribbon-health)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fnikothomas%2Fribbon-health.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fnikothomas%2Fribbon-health?ref=badge_shield)
[![Documentation](https://docs.rs/ribbon-health/badge.svg)](https://docs.rs/ribbon-health)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project along with some custom tooling where necessary.

- API version: 2.2
- Package version: 2.0.0
- Generator version: 7.12.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

### From crates.io

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
ribbon-health = "2.0.0"
```

### With specific features

The crate offers optional features:

```toml
[dependencies]
ribbon-health = { version = "2.0.0", features = ["bon"] }
```

### From GitHub repository

You can also add the dependency directly from this GitHub repository:

```toml
[dependencies]
ribbon-health = { git = "https://github.com/nikothomas/ribbon-health-rs", branch = "main" }
```

### Local Development

If you're developing with a local copy, put the package under your project folder in a directory named `ribbon-health` and add the following to `Cargo.toml` under `[dependencies]`:

```toml
ribbon-health = { path = "./ribbon-health" }
```

## Usage Examples

### Basic Setup

```rust
use std::sync::Arc;
use ribbon_health::{
    apis::{
        configuration::Configuration,
        ApiClient, Api, providers_api::GetCustomProvidersParams
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a configuration with your Ribbon Health API token
    let mut config = Configuration::default();
    config.bearer_access_token = Some("YOUR_API_TOKEN".to_string());
    
    // Create an API client with the configuration
    let client = ApiClient::new(Arc::new(config));
    
    // Now you can use any of the API endpoints
    let providers_api = client.providers_api();
    
    // Example: Search for healthcare providers
    let params = GetCustomProvidersParams {
        specialty: Some("Cardiology".to_string()),
        state: Some("NY".to_string()),
        page_size: Some(10),
        page: Some(1),
        // Optional parameters can be set as needed
        min_rating: Some(4),
        // Set other fields to None
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
        address: None,
        location_ids: None,
        _excl_location_ids: None,
        location: None,
        min_location_confidence: None,
        min_confidence: None,
        distance: None,
        insurance_ids: None,
        _excl_insurance_ids: None,
        insurance_carrier_name: None,
        location_insurance_ids: None,
        _excl_location_insurance_ids: None,
        national_bluecard: None,
        specialty_ids: None,
        _excl_specialty_ids: None,
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
    let response = providers_api.get_custom_providers(params).await?;
    
    // Process the response
    match response.entity {
        Some(entity) => {
            println!("Found {} providers", entity.data.len());
            for provider in entity.data {
                println!("Provider: {} {} (NPI: {})", 
                    provider.first_name.unwrap_or_default(), 
                    provider.last_name.unwrap_or_default(),
                    provider.npi);
            }
        },
        None => println!("No providers found"),
    }
    
    Ok(())
}
```

### Get Provider by NPI

```rust
use std::sync::Arc;
use ribbon_health::{
    apis::{
        configuration::Configuration,
        ApiClient, Api, providers_api::GetCustomProviderParams
    },
};

async fn get_provider_by_npi(npi: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration and client
    let mut config = Configuration::default();
    config.bearer_access_token = Some("YOUR_API_TOKEN".to_string());
    let client = ApiClient::new(Arc::new(config));
    
    // Set up parameters
    let params = GetCustomProviderParams {
        npi: npi.to_string(),
        max_insurances: Some(10),
    };
    
    // Make the API call
    let response = client.providers_api().get_custom_provider(params).await?;
    
    // Process the result
    if let Some(provider) = response.entity {
        println!("Provider details: {}", provider.npi);
        if let Some(locations) = provider.locations {
            println!("Practice at {} locations", locations.len());
            for location in locations {
                println!("- {}", location.address.unwrap_or_default());
            }
        }
    }
    
    Ok(())
}
```

### Using the Builder Pattern with the `bon` Feature

The `bon` feature adds builder pattern support to parameter structs, making it easier to construct API requests with many optional parameters:

```rust
use std::sync::Arc;
use ribbon_health::{
    apis::{
        configuration::Configuration,
        ApiClient, Api, providers_api::GetCustomProvidersParams
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a configuration with your Ribbon Health API token
    let mut config = Configuration::default();
    config.bearer_access_token = Some("YOUR_API_TOKEN".to_string());
    
    // Create an API client with the configuration
    let client = ApiClient::new(Arc::new(config));
    
    // Use the builder pattern to construct parameters
    // This is much cleaner than setting dozens of fields to None
    let params = GetCustomProvidersParams::builder()
        .specialty(Some("Cardiology".to_string()))
        .state(Some("NY".to_string()))
        .page_size(Some(10))
        .page(Some(1))
        .min_rating(Some(4))
        .gender(Some("f".to_string()))  // Only female providers
        .insurance_carrier_name(Some("Aetna".to_string()))
        .distance(Some(25.0))  // Within 25 miles
        .location(Some("40.7128,-74.0060".to_string()))  // NYC coordinates
        .build();
    
    // Execute the API call
    let response = client.providers_api().get_custom_providers(params).await?;
    
    // Process the response
    if let Some(entity) = response.entity {
        println!("Found {} providers", entity.data.len());
        // Process providers...
    }
    
    Ok(())
}
```



