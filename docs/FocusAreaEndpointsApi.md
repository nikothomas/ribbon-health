# \FocusAreaEndpointsApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_clinical_area**](FocusAreaEndpointsApi.md#get_clinical_area) | **GET** /custom/clinical_areas/{clinical_area_uuid} | getClinicalArea
[**get_clinical_areas**](FocusAreaEndpointsApi.md#get_clinical_areas) | **GET** /custom/clinical_areas | getClinicalAreas
[**get_condition**](FocusAreaEndpointsApi.md#get_condition) | **GET** /custom/conditions/{condition_uuid} | getCondition
[**get_conditions**](FocusAreaEndpointsApi.md#get_conditions) | **GET** /custom/conditions | getConditions
[**get_treatment**](FocusAreaEndpointsApi.md#get_treatment) | **GET** /custom/treatments/{treatment_uuid} | getTreatment
[**get_treatments**](FocusAreaEndpointsApi.md#get_treatments) | **GET** /custom/treatments | getTreatments



## get_clinical_area

> models::GetCustomProviders200ResponseParametersClinicalAreaIds get_clinical_area(clinical_area_uuid)
getClinicalArea

Retrieve data on a specific clinical area. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clinical_area_uuid** | **uuid::Uuid** | The UUID of the target clinical area. | [required] |

### Return type

[**models::GetCustomProviders200ResponseParametersClinicalAreaIds**](getCustomProviders_200_response_parameters_clinical_area_ids.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clinical_areas

> models::GetClinicalAreas200Response get_clinical_areas(page, page_size, search, clinical_area, _excl_clinical_area_ids, specialty_ids, condition, condition_ids, treatment, treatment_ids, r#type)
getClinicalAreas

Returns clinical areas that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |
**search** | Option<**String**> | String input that fuzzy searches against key fields within each clinical area object to return the most relevant options. |  |
**clinical_area** | Option<**String**> | String input that fuzzy searches on `display` field. |  |
**_excl_clinical_area_ids** | Option<**String**> | Comma separated list of clinical area UUIDs to exclude from search results. |  |
**specialty_ids** | Option<**String**> | Comma separated list of specialty UUIDs. |  |
**condition** | Option<**String**> | String input that fuzzy searches on `condition.display` field. |  |
**condition_ids** | Option<**String**> | Comma separated list of condition UUIDs. |  |
**treatment** | Option<**String**> | String input that fuzzy searches on `treatment.display` field. |  |
**treatment_ids** | Option<**String**> | Comma separated list of treatment UUIDs. |  |
**r#type** | Option<**String**> | String input of the type of clinical areas to return. Options for input are either `providers` or `locations`. Note: Defaults to returning all clinical areas of any type. |  |

### Return type

[**models::GetClinicalAreas200Response**](getClinicalAreas_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_condition

> models::GetCustomProviders200ResponseParametersClinicalAreaIdsConditionsInner get_condition(condition_uuid)
getCondition

Retrieve data on a specific condition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**condition_uuid** | **uuid::Uuid** | The UUID of the target condition. | [required] |

### Return type

[**models::GetCustomProviders200ResponseParametersClinicalAreaIdsConditionsInner**](getCustomProviders_200_response_parameters_clinical_area_ids_conditions_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conditions

> models::GetConditions200Response get_conditions(page, page_size, search, _excl_condition_ids, specialty_ids, module)
getConditions

Returns conditions that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |
**search** | Option<**String**> | String input that fuzzy searches against key fields within each condition object to return the most relevant options. |  |
**_excl_condition_ids** | Option<**String**> | Comma separated list of condition UUIDs to exclude from search results. |  |
**specialty_ids** | Option<**String**> | Comma separated list of specialty UUIDs. |  |
**module** | Option<**String**> | String input of the type of clinical areas to return. Options for input are either `focus_areas` or `condition_cost_estimate`. Note: This input must be an exact string match to work |  |

### Return type

[**models::GetConditions200Response**](getConditions_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_treatment

> models::GetCustomProviders200ResponseParametersClinicalAreaIdsTreatmentsInner get_treatment(treatment_uuid)
getTreatment

Retrieve data on a specific treatment. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**treatment_uuid** | **uuid::Uuid** | The UUID of the target treatment. | [required] |

### Return type

[**models::GetCustomProviders200ResponseParametersClinicalAreaIdsTreatmentsInner**](getCustomProviders_200_response_parameters_clinical_area_ids_treatments_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_treatments

> models::GetTreatments200Response get_treatments(page, page_size, search, _excl_treatment_ids, specialty_ids, r#type)
getTreatments

Returns treatments that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |
**search** | Option<**String**> | String input that fuzzy searches against key fields within each treatment object to return the most relevant options. |  |
**_excl_treatment_ids** | Option<**String**> | Comma separated list of treatment UUIDs to exclude from search results. |  |
**specialty_ids** | Option<**String**> | Comma separated list of specialty UUIDs. |  |
**r#type** | Option<**String**> | String input of the type of clinical areas to return. Options for input are either `providers` or `locations`. Note: Defaults to returning all clinical areas of any type. |  |

### Return type

[**models::GetTreatments200Response**](getTreatments_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

