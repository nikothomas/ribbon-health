# \PriceTransparencyApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pricing_carrier**](PriceTransparencyApi.md#get_pricing_carrier) | **GET** /pricing/carrier/{carrier_uuid} | getPricingCarrier
[**get_pricing_carrier_names**](PriceTransparencyApi.md#get_pricing_carrier_names) | **GET** /pricing/version | getPricingCarrierNames
[**get_pricing_carriers**](PriceTransparencyApi.md#get_pricing_carriers) | **GET** /pricing/carriers | getPricingCarriers
[**get_pricing_provider_procedure**](PriceTransparencyApi.md#get_pricing_provider_procedure) | **GET** /pricing/providers/{npi}/procedures/{procedure_uuid} | getPricingProviderProcedure
[**get_pricing_provider_procedure_location**](PriceTransparencyApi.md#get_pricing_provider_procedure_location) | **GET** /pricing/providers/{npi}/procedures/{procedure_uuid}/locations/{location_uuid} | getPricingProviderProcedureLocation
[**get_pricing_provider_procedures**](PriceTransparencyApi.md#get_pricing_provider_procedures) | **GET** /pricing/providers/{npi}/procedures | getPricingProviderProcedures
[**get_pricing_providers**](PriceTransparencyApi.md#get_pricing_providers) | **GET** /pricing/providers | getPricingProviders
[**get_pricing_version_carrier**](PriceTransparencyApi.md#get_pricing_version_carrier) | **GET** /pricing/version/{carrier_name} | getPricingVersionCarrier



## get_pricing_carrier

> models::GetPricingCarriers200ResponseDataInner get_pricing_carrier(carrier_uuid)
getPricingCarrier

Fetch metadata including the recency of the pricing data used for a specific carrier. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**carrier_uuid** | **uuid::Uuid** | The UUID of the insurance carrier. | [required] |

### Return type

[**models::GetPricingCarriers200ResponseDataInner**](getPricingCarriers_200_response_data_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_carrier_names

> models::GetPricingCarrierNames200Response get_pricing_carrier_names()
getPricingCarrierNames

This endpoint will the names of the carriers for which we have data. This can be used to fetch the recency of the data used per carrier.  This endpoint is deprecated. Please use [List Carriers](./getpricingcarriers) instead. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPricingCarrierNames200Response**](getPricingCarrierNames_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_carriers

> models::GetPricingCarriers200Response get_pricing_carriers()
getPricingCarriers

This endpoint will show the carriers for which we have data. This can be used to fetch the recency of the data used per carrier. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPricingCarriers200Response**](getPricingCarriers_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_provider_procedure

> models::GetPricingProviderProcedure200Response get_pricing_provider_procedure(npi, procedure_uuid, plan_id, page, page_size)
getPricingProviderProcedure

Find the prices offered by a single provider for a specific procedure, with a given insurance, across practice locations.  #### Example Use Case Compare insurance-specific price estimates of a Leg MRI for a single provider at multiple relevant practices (e.g., compare this provider's rates when performing the procedure at both the provider's private outpatient facility, as well as a nearby hospital system clinic where they also practice). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**procedure_uuid** | **uuid::Uuid** | The UUID of the target procedure. If the target procedure is a procedure bundle but we do not have any data for it, we will return data for its anchor procedure instead. | [required] |
**plan_id** | Option<**uuid::Uuid**> | Search for negotiated rates for the insurance plan with this UUID. |  |
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |

### Return type

[**models::GetPricingProviderProcedure200Response**](getPricingProviderProcedure_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_provider_procedure_location

> models::GetPricingProviderProcedureLocation200Response get_pricing_provider_procedure_location(npi, procedure_uuid, location_uuid, plan_id)
getPricingProviderProcedureLocation

Search for a price estimate for a specific procedure from a specific provider at a specific location, with a given insurance plan.  #### Example Use Case Given an insurance, identify the expected price of a particular procedure from a specific provider at a known facility. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**procedure_uuid** | **uuid::Uuid** | The UUID of the target procedure. If the target procedure is a procedure bundle but we do not have any data for it, we will return data for its anchor procedure instead. | [required] |
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**plan_id** | Option<**uuid::Uuid**> | Search for negotiated rates for the insurance plan with this UUID. |  |

### Return type

[**models::GetPricingProviderProcedureLocation200Response**](getPricingProviderProcedureLocation_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_provider_procedures

> models::GetPricingProviderProcedures200Response get_pricing_provider_procedures(npi, plan_id, page, page_size)
getPricingProviderProcedures

Fetch the list of procedures that a single provider performs, with the lowest available negotiated rates specific to a given insurance for each procedure.  #### Example Use Case For a given provider, search the full list of procedures that they are likely to perform where there are negotiated rates available for a particular insurance, and return the minimum price for each procedure. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**npi** | **String** | The 10-digit National Provider Identifier (NPI) of the healthcare provider to fetch. | [required] |
**plan_id** | Option<**uuid::Uuid**> | Search for negotiated rates for the insurance plan with this UUID. |  |
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |

### Return type

[**models::GetPricingProviderProcedures200Response**](getPricingProviderProcedures_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_providers

> models::GetPricingProviders200Response get_pricing_providers(procedure_id, procedure, procedure_code, page, page_size, plan_id, specialty_ids, specialty, address, location, distance, fields, _excl_fields)
getPricingProviders

Search for providers that perform a given procedure and find the lowest insurance-specific price for a procedure in your area.  #### Example Use Case Search for all applicable provider negotiated rates, given a specific insurance and procedure (and optionally, a specific location/address and distance). For example, search for all providers near me who perform Leg MRIs and who take a given insurance, sorted by lowest price. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**procedure_id** | Option<**uuid::Uuid**> | Search for prices for the procedure with the given UUID.  If the given ID is for a procedure bundle but do not have data for the insurance specified by the `plan_id` parameter, we will return data for its anchor procedure instead.  Exactly one of `procedure`, `procedure_id`, or `procedure_code` must be specified. |  |
**procedure** | Option<**String**> | Search for prices for the given procedure. This input is fuzzy matched to the most relevant procedure `display` field.  We will preferentially match procedure bundles. If we match a procedure bundle but do not have data for the insurance specified by the `plan_id` parameter, we will return data for its anchor procedure instead.  Exactly one of `procedure`, `procedure_id`, or `procedure_code` must be specified. |  |
**procedure_code** | Option<**String**> | Search for prices for procedures with the given billing code.  We will preferentially match procedure bundles. If we match a procedure bundle but do not have data for the insurance specified by the `plan_id` parameter, we will return data for its anchor procedure instead.  Exactly one of `procedure`, `procedure_id`, or `procedure_code` must be specified. |  |
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |
**plan_id** | Option<**uuid::Uuid**> | Search for negotiated rates for the insurance plan with this UUID. |  |
**specialty_ids** | Option<**String**> | A comma separated list of specialty UUIDs. Filter to providers with any of the given specialties. |  |
**specialty** | Option<**String**> | String input of a provider specialty that will be interpreted and matched to the single closest specialty, dealing with basic typos and colloquial names for providers. |  |
**address** | Option<**String**> | String input of an address that will be interpreted and geocoded in real time. |  |
**location** | Option<**String**> | Latitude/longitude pair of coordinates in lieu of a string address. |  |
**distance** | Option<**i32**> | The proximity radius of providers returned. |  |
**fields** | Option<**String**> | Comma-separated list of fields within the provider object to return. Can be used to greatly reduce the size of the response by requesting only data you intend to use.  Note that all price information is nested under the `matched_location` field. You almost certainly want to return this field.  Cannot be used in tandem with `_excl_fields`. |  |
**_excl_fields** | Option<**String**> | Comma-separated list of fields within the provider object to exclude from the response. Can be used to greatly reduce the size of the response by requesting only data you intend to use.  Cannot be used in tandem with `fields`. |  |

### Return type

[**models::GetPricingProviders200Response**](getPricingProviders_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_version_carrier

> models::GetPricingVersionCarrier200Response get_pricing_version_carrier(carrier_name)
getPricingVersionCarrier

Fetch the recency of the pricing data used for a specific carrier by name.  This endpoint is deprecated. Please use [Get Carrier](./getpricingcarrier) instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**carrier_name** | **String** | The name of the insurance carrier. | [required] |

### Return type

[**models::GetPricingVersionCarrier200Response**](getPricingVersionCarrier_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

