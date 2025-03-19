# \FiltersApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_location_filter**](FiltersApi.md#create_custom_location_filter) | **POST** /custom/locations/filters | createCustomLocationFilter
[**create_custom_provider_filter**](FiltersApi.md#create_custom_provider_filter) | **POST** /custom/providers/filters | createCustomProviderFilter
[**delete_custom_location_filter**](FiltersApi.md#delete_custom_location_filter) | **DELETE** /custom/locations/filters/{filter_uuid} | deleteCustomLocationFilter
[**delete_custom_provider_filter**](FiltersApi.md#delete_custom_provider_filter) | **DELETE** /custom/providers/filters/{filter_uuid} | deleteCustomProviderFilter
[**edit_custom_location_filter**](FiltersApi.md#edit_custom_location_filter) | **PUT** /custom/locations/filters/{filter_uuid} | editCustomLocationFilter
[**edit_custom_provider_filter**](FiltersApi.md#edit_custom_provider_filter) | **PUT** /custom/providers/filters/{filter_uuid} | editCustomProviderFilter
[**get_custom_location_filters**](FiltersApi.md#get_custom_location_filters) | **GET** /custom/locations/filters | getCustomLocationFilters
[**get_custom_provider_filters**](FiltersApi.md#get_custom_provider_filters) | **GET** /custom/providers/filters | getCustomProviderFilters



## create_custom_location_filter

> models::CreateCustomProviderFilter201Response create_custom_location_filter(create_custom_provider_filter_request)
createCustomLocationFilter

Create new filters to be used when searching for locations in the [Search Locations](./getcustomlocations) endpoint.  You can create filters for both Ribbon's existing data fields as well as any custom data fields you create.  #### Example Use Case: Let's say you add a new field to a bunch of locations that match your use case, for instance `c_section_rate`. You could make this field searchable so that your search through [Search Locations](./getcustomlocations) when you pass through a certain parameter that specifies a maximum threshold. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_custom_provider_filter_request** | [**CreateCustomProviderFilterRequest**](CreateCustomProviderFilterRequest.md) | The new filter to create | [required] |

### Return type

[**models::CreateCustomProviderFilter201Response**](createCustomProviderFilter_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_custom_provider_filter

> models::CreateCustomProviderFilter201Response create_custom_provider_filter(create_custom_provider_filter_request)
createCustomProviderFilter

Create new filters to be used when searching for providers in the [Search Providers](./getcustomproviders) endpoint.  You can create filters for both Ribbon's existing data fields as well as any custom data fields you create.  #### Example Use Case: Let's say you add a new field to a bunch of providers that match your use case, for instance `c_section_rate`. You could make this field searchable so that your search through [Search Providers](./getcustomproviders) when you pass through a certain parameter that specifies a maximum threshold. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_custom_provider_filter_request** | [**CreateCustomProviderFilterRequest**](CreateCustomProviderFilterRequest.md) | The new filter to create | [required] |

### Return type

[**models::CreateCustomProviderFilter201Response**](createCustomProviderFilter_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_location_filter

> delete_custom_location_filter(filter_uuid)
deleteCustomLocationFilter

Delete a location filter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_uuid** | **uuid::Uuid** | The UUID of the filter you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_provider_filter

> delete_custom_provider_filter(filter_uuid)
deleteCustomProviderFilter

Delete a provider filter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_uuid** | **uuid::Uuid** | The UUID of the filter you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_custom_location_filter

> models::EditCustomLocationFilter200Response edit_custom_location_filter(filter_uuid, edit_custom_location_filter_request)
editCustomLocationFilter

Edit any of the fields in a custom location filter you have already created. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_uuid** | **uuid::Uuid** | The UUID of the filter you want to edit. | [required] |
**edit_custom_location_filter_request** | [**EditCustomLocationFilterRequest**](EditCustomLocationFilterRequest.md) | A JSON object mapping the name of the field to update to its new value. | [required] |

### Return type

[**models::EditCustomLocationFilter200Response**](editCustomLocationFilter_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_custom_provider_filter

> models::CreateCustomProviderFilter201Response edit_custom_provider_filter(filter_uuid, edit_custom_provider_filter_request)
editCustomProviderFilter

Edit any of the fields in a custom provider filter you have already created. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_uuid** | **uuid::Uuid** | The UUID of the filter you want to edit. | [required] |
**edit_custom_provider_filter_request** | [**EditCustomProviderFilterRequest**](EditCustomProviderFilterRequest.md) | A JSON object mapping the name of the field to update to its new value. | [required] |

### Return type

[**models::CreateCustomProviderFilter201Response**](createCustomProviderFilter_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_location_filters

> models::GetCustomProviderFilters200Response get_custom_location_filters()
getCustomLocationFilters

Fetch all previously created custom filters for locations. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCustomProviderFilters200Response**](getCustomProviderFilters_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_provider_filters

> models::GetCustomProviderFilters200Response get_custom_provider_filters()
getCustomProviderFilters

Fetch all previously created filters for providers. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCustomProviderFilters200Response**](getCustomProviderFilters_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

