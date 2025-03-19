# \ProvidersApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_custom_provider**](ProvidersApi.md#get_custom_provider) | **GET** /custom/providers/{npi} | getCustomProvider
[**get_custom_providers**](ProvidersApi.md#get_custom_providers) | **GET** /custom/providers | getCustomProviders
[**put_custom_provider**](ProvidersApi.md#put_custom_provider) | **PUT** /custom/providers/{npi} | putCustomProvider
[**put_custom_provider_clinical_areas**](ProvidersApi.md#put_custom_provider_clinical_areas) | **PUT** /custom/providers/{npi}/clinical_areas | putCustomProviderClinicalAreas
[**put_custom_provider_location**](ProvidersApi.md#put_custom_provider_location) | **PUT** /custom/providers/{npi}/locations/{location_uuid} | putCustomProviderLocation
[**put_custom_provider_location_insurances**](ProvidersApi.md#put_custom_provider_location_insurances) | **PUT** /custom/providers/{npi}/locations/{location_uuid}/insurances | putCustomProviderLocationInsurances
[**put_custom_provider_location_organizations**](ProvidersApi.md#put_custom_provider_location_organizations) | **PUT** /custom/providers/{npi}/locations/{location_uuid}/organizations | putCustomProviderLocationOrganizations
[**put_custom_provider_locations**](ProvidersApi.md#put_custom_provider_locations) | **PUT** /custom/providers/{npi}/locations | putCustomProviderLocations
[**put_custom_provider_primary_specialties**](ProvidersApi.md#put_custom_provider_primary_specialties) | **PUT** /custom/providers/{npi}/specialties/{specialty_uuid} | putCustomProviderPrimarySpecialties
[**put_custom_provider_procedures**](ProvidersApi.md#put_custom_provider_procedures) | **PUT** /custom/providers/{npi}/procedures | putCustomProviderProcedures
[**put_custom_provider_specialties**](ProvidersApi.md#put_custom_provider_specialties) | **PUT** /custom/providers/{npi}/specialties | putCustomProviderSpecialties



## get_custom_provider

> models::GetCustomProvider200Response get_custom_provider(npi, max_insurances)
getCustomProvider

Retrieve detailed information for any provider given their NPI, such as locations, contact information, education, patient satisfaction, etc. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**max_insurances** | Option<**i32**> | If provided, returns only up to this many insurances per provider. Useful to limit the amount of data returned when you do not need the entire list of accepted insurances. |  |

### Return type

[**models::GetCustomProvider200Response**](getCustomProvider_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_providers

> models::GetCustomProviders200Response get_custom_providers(page, page_size, max_locations, fields, _excl_fields, npis, name, provider_types, _excl_provider_types, gender, max_age, min_age, language, _excl_language, min_rating, address, location_ids, _excl_location_ids, location, min_location_confidence, min_confidence, distance, state, insurance_ids, _excl_insurance_ids, insurance_carrier_name, location_insurance_ids, _excl_location_insurance_ids, national_bluecard, specialty_ids, _excl_specialty_ids, specialty, specialty_ids_primary, _excl_specialty_ids_primary, specialty_primary, apply_specialty_grouping, procedure_ids, _excl_procedure_ids, procedure, min_experience_index, max_cost_index, clinical_area, clinical_area_ids, _excl_clinical_area_ids, condition, condition_ids, _excl_condition_ids, treatment, treatment_ids, _excl_treatment_ids, panel_ages, _excl_panel_ages, panel_sexes, min_outcomes_index, min_efficiency_index, max_unit_cost_index, max_ribbon_cost_score, location_organization_ids, _excl_location_organization_ids, tin_ids, tin_name, tin_legal_name)
getCustomProviders

Allows you to quickly list doctors based on important search criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. |  |
**page_size** | Option<**i32**> | Number of items per page. |  |
**max_locations** | Option<**i32**> | Max number of locations returned per provider. |  |
**fields** | Option<**String**> | Comma-separated list of fields to include. |  |
**_excl_fields** | Option<**String**> | Comma-separated list of fields to exclude. |  |
**npis** | Option<**String**> | A comma-separated list of NPIs to filter on. |  |
**name** | Option<**String**> | Provider’s name or partial name to search. |  |
**provider_types** | Option<**String**> | Comma-separated list of provider types to include. |  |
**_excl_provider_types** | Option<**String**> | Comma-separated list of provider types to exclude. |  |
**gender** | Option<**String**> | Filter providers by gender (e.g., 'm', 'f'). |  |
**max_age** | Option<**i32**> | Maximum provider age to include. |  |
**min_age** | Option<**i32**> | Minimum provider age to include. |  |
**language** | Option<**String**> | Provider’s language (or comma-separated languages). |  |
**_excl_language** | Option<**String**> | Language(s) to exclude. |  |
**min_rating** | Option<**i32**> | Minimum star rating allowed. |  |
**address** | Option<**String**> | Free-text address to search. |  |
**location_ids** | Option<**String**> | Comma-separated list of location IDs to include. |  |
**_excl_location_ids** | Option<**String**> | Comma-separated list of location IDs to exclude. |  |
**location** | Option<**String**> | A latitude,longitude pair for proximity searching. |  |
**min_location_confidence** | Option<**i32**> | Minimum location confidence score to include. |  |
**min_confidence** | Option<**i32**> | (Synonym to min_location_confidence, if used.) |  |
**distance** | Option<**i32**> | Distance in miles from the given location. |  |
**state** | Option<**String**> | Two-letter US state code (e.g., \"NY\"). |  |
**insurance_ids** | Option<**String**> | Comma-separated insurance IDs to include. |  |
**_excl_insurance_ids** | Option<**String**> | Comma-separated insurance IDs to exclude. |  |
**insurance_carrier_name** | Option<**String**> | Filter providers by insurance carrier name. |  |
**location_insurance_ids** | Option<**String**> | Comma-separated insurance IDs applicable at certain locations. |  |
**_excl_location_insurance_ids** | Option<**String**> | Comma-separated insurance IDs to exclude at locations. |  |
**national_bluecard** | Option<**bool**> | True if searching for BlueCard or national Blue network coverage. |  |
**specialty_ids** | Option<**String**> | Comma-separated list of specialty IDs to include. |  |
**_excl_specialty_ids** | Option<**String**> | Comma-separated list of specialty IDs to exclude. |  |
**specialty** | Option<**String**> | Specialty name to include. |  |
**specialty_ids_primary** | Option<**String**> | Comma-separated list of primary specialty IDs to include. |  |
**_excl_specialty_ids_primary** | Option<**String**> | Comma-separated list of primary specialty IDs to exclude. |  |
**specialty_primary** | Option<**String**> | Primary specialty name to include. |  |
**apply_specialty_grouping** | Option<**bool**> | Whether to group related specialties automatically. |  |
**procedure_ids** | Option<**String**> | Comma-separated list of procedure IDs to include. |  |
**_excl_procedure_ids** | Option<**String**> | Comma-separated list of procedure IDs to exclude. |  |
**procedure** | Option<**String**> | Procedure name to include. |  |
**min_experience_index** | Option<**i32**> | Minimum experience index of providers for the given procedure. |  |
**max_cost_index** | Option<**i32**> | Maximum allowed cost index for the given procedure. |  |
**clinical_area** | Option<**String**> | Name of the clinical area to filter on. |  |
**clinical_area_ids** | Option<**String**> | Comma-separated clinical area IDs to include. |  |
**_excl_clinical_area_ids** | Option<**String**> | Comma-separated clinical area IDs to exclude. |  |
**condition** | Option<**String**> | Name of the condition to filter on. |  |
**condition_ids** | Option<**String**> | Comma-separated condition IDs to include. |  |
**_excl_condition_ids** | Option<**String**> | Comma-separated condition IDs to exclude. |  |
**treatment** | Option<**String**> | Treatment name to filter on. |  |
**treatment_ids** | Option<**String**> | Comma-separated treatment IDs to include. |  |
**_excl_treatment_ids** | Option<**String**> | Comma-separated treatment IDs to exclude. |  |
**panel_ages** | Option<**String**> | Comma-separated age panels to include. |  |
**_excl_panel_ages** | Option<**String**> | Comma-separated age panels to exclude. |  |
**panel_sexes** | Option<**String**> | Filter for sexes or gender categories (e.g., 'Primarily female'). |  |
**min_outcomes_index** | Option<**i32**> | Minimal acceptable outcomes score/index. |  |
**min_efficiency_index** | Option<**i32**> | Minimal acceptable efficiency score/index. |  |
**max_unit_cost_index** | Option<**i32**> | Maximum allowable unit cost index. |  |
**max_ribbon_cost_score** | Option<**i32**> | Maximum allowable \"ribbon\" cost score. |  |
**location_organization_ids** | Option<**String**> | Comma-separated organization IDs for location matching. |  |
**_excl_location_organization_ids** | Option<**String**> | Comma-separated organization IDs to exclude. |  |
**tin_ids** | Option<**String**> | Comma-separated TIN IDs to include. |  |
**tin_name** | Option<**String**> | TIN name or partial name to include. |  |
**tin_legal_name** | Option<**String**> | TIN's legal name to include. |  |

### Return type

[**models::GetCustomProviders200Response**](getCustomProviders_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider

> models::PutCustomProvider200Response put_custom_provider(npi, put_custom_provider_request)
putCustomProvider

Edit all fields that do not fall under `specialties`, `locations`, or `insurances`. You may also add new fields or remove existing fields.  #### Looking For The Old Documentation? We're in the process of revamping our documentation. You can find the old page for this endpoint [here](https://ribbon.readme.io/docs/add-or-edit-provider-fields-old). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**put_custom_provider_request** | [**PutCustomProviderRequest**](PutCustomProviderRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PutCustomProvider200Response**](putCustomProvider_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_clinical_areas

> models::PutCustomProviderLocations200Response put_custom_provider_clinical_areas(npi, put_custom_provider_clinical_areas_request)
putCustomProviderClinicalAreas

Add or remove clinical areas for a provider using our standard clinical area UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**put_custom_provider_clinical_areas_request** | [**PutCustomProviderClinicalAreasRequest**](PutCustomProviderClinicalAreasRequest.md) | A set of instructions for how to update the provider's clinical areas. | [required] |

### Return type

[**models::PutCustomProviderLocations200Response**](putCustomProviderLocations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_location

> models::PutCustomProvider200Response put_custom_provider_location(npi, location_uuid, put_custom_provider_request)
putCustomProviderLocation

Edit all fields that do not fall under `uuid`, `google_maps_link`, `latitude`, or `longitude`. You may also add new fields or remove existing fields. These updates are provider-specific and will not affect other providers practicing at the same location. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_provider_request** | [**PutCustomProviderRequest**](PutCustomProviderRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PutCustomProvider200Response**](putCustomProvider_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_location_insurances

> models::PutCustomProviderLocations200Response put_custom_provider_location_insurances(npi, location_uuid, put_custom_provider_location_insurances_request)
putCustomProviderLocationInsurances

Add or remove insurances accepted by a provider at a specific location using our standard insurance UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_provider_location_insurances_request** | [**PutCustomProviderLocationInsurancesRequest**](PutCustomProviderLocationInsurancesRequest.md) | A set of instructions for how to update the provider's insurances at this location. | [required] |

### Return type

[**models::PutCustomProviderLocations200Response**](putCustomProviderLocations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_location_organizations

> models::PutCustomProviderLocations200Response put_custom_provider_location_organizations(npi, location_uuid, put_custom_provider_location_organizations_request)
putCustomProviderLocationOrganizations

Add or remove organizations accepted by a provider at a specific location using our standard organization UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_provider_location_organizations_request** | [**PutCustomProviderLocationOrganizationsRequest**](PutCustomProviderLocationOrganizationsRequest.md) | A set of instructions for how to update the provider's organizations at this location. | [required] |

### Return type

[**models::PutCustomProviderLocations200Response**](putCustomProviderLocations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_locations

> models::PutCustomProviderLocations200Response put_custom_provider_locations(npi, put_custom_provider_locations_request)
putCustomProviderLocations

Add or remove locations a provider practices at using our standard location UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**put_custom_provider_locations_request** | [**PutCustomProviderLocationsRequest**](PutCustomProviderLocationsRequest.md) | A set of instructions for how to update the provider's locations. | [required] |

### Return type

[**models::PutCustomProviderLocations200Response**](putCustomProviderLocations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_primary_specialties

> models::PutCustomProviderPrimarySpecialties200Response put_custom_provider_primary_specialties(npi, specialty_uuid, put_custom_provider_primary_specialties_request)
putCustomProviderPrimarySpecialties

Edit whether a single specialty is one of the provider's primary specialties. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**specialty_uuid** | **uuid::Uuid** | The UUID of the target specialty. | [required] |
**put_custom_provider_primary_specialties_request** | [**PutCustomProviderPrimarySpecialtiesRequest**](PutCustomProviderPrimarySpecialtiesRequest.md) | Whether or not this specialty is a primary specialty. | [required] |

### Return type

[**models::PutCustomProviderPrimarySpecialties200Response**](putCustomProviderPrimarySpecialties_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_procedures

> models::PutCustomProviderLocations200Response put_custom_provider_procedures(npi, put_custom_provider_procedures_request)
putCustomProviderProcedures

Add or remove procedures for a provider using our standard procedure UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**put_custom_provider_procedures_request** | [**PutCustomProviderProceduresRequest**](PutCustomProviderProceduresRequest.md) | A set of instructions for how to update the provider's procedures. | [required] |

### Return type

[**models::PutCustomProviderLocations200Response**](putCustomProviderLocations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_specialties

> models::PutCustomProviderSpecialties200Response put_custom_provider_specialties(npi, put_custom_provider_specialties_request)
putCustomProviderSpecialties

Add or remove specialties for a provider using our standard specialty UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**put_custom_provider_specialties_request** | [**PutCustomProviderSpecialtiesRequest**](PutCustomProviderSpecialtiesRequest.md) | A set of instructions for how to update the provider's specialties. | [required] |

### Return type

[**models::PutCustomProviderSpecialties200Response**](putCustomProviderSpecialties_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

