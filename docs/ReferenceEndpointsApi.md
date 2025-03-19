# \ReferenceEndpointsApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_custom_insurance**](ReferenceEndpointsApi.md#delete_custom_insurance) | **DELETE** /custom/insurances/{insurance_uuid} | deleteCustomInsurance
[**delete_custom_location_type**](ReferenceEndpointsApi.md#delete_custom_location_type) | **DELETE** /custom/location_types/{location_type_uuid} | deleteCustomLocationType
[**delete_custom_provider_type**](ReferenceEndpointsApi.md#delete_custom_provider_type) | **DELETE** /custom/provider_types/{provider_type_uuid} | deleteCustomProviderType
[**delete_custom_specialty**](ReferenceEndpointsApi.md#delete_custom_specialty) | **DELETE** /custom/specialties/{specialty_uuid} | deleteCustomSpecialty
[**get_custom_insurance**](ReferenceEndpointsApi.md#get_custom_insurance) | **GET** /custom/insurances/{insurance_uuid} | getCustomInsurance
[**get_custom_location_type**](ReferenceEndpointsApi.md#get_custom_location_type) | **GET** /custom/location_types/{location_type_uuid} | getCustomLocationType
[**get_custom_location_types**](ReferenceEndpointsApi.md#get_custom_location_types) | **GET** /location_types | getCustomLocationTypes
[**get_custom_provider_type**](ReferenceEndpointsApi.md#get_custom_provider_type) | **GET** /custom/provider_types/{provider_type_uuid} | getCustomProviderType
[**get_custom_provider_types**](ReferenceEndpointsApi.md#get_custom_provider_types) | **GET** /provider_types | getCustomProviderTypes
[**get_custom_specialty**](ReferenceEndpointsApi.md#get_custom_specialty) | **GET** /custom/specialties/{specialty_uuid} | getCustomSpecialty
[**get_insurances**](ReferenceEndpointsApi.md#get_insurances) | **GET** /custom/insurances | getInsurances
[**get_languages**](ReferenceEndpointsApi.md#get_languages) | **GET** /languages | getLanguages
[**get_procedure**](ReferenceEndpointsApi.md#get_procedure) | **GET** /procedures/{procedure_uuid} | getProcedure
[**get_procedures**](ReferenceEndpointsApi.md#get_procedures) | **GET** /procedures | getProcedures
[**get_specialties**](ReferenceEndpointsApi.md#get_specialties) | **GET** /custom/specialties | getSpecialties
[**post_custom_insurance**](ReferenceEndpointsApi.md#post_custom_insurance) | **POST** /custom/insurances | postCustomInsurance
[**post_custom_location_type**](ReferenceEndpointsApi.md#post_custom_location_type) | **POST** /custom/location_types | postCustomLocationType
[**post_custom_provider_type**](ReferenceEndpointsApi.md#post_custom_provider_type) | **POST** /custom/provider_types | postCustomProviderType
[**post_custom_specialty**](ReferenceEndpointsApi.md#post_custom_specialty) | **POST** /custom/specialties | postCustomSpecialty
[**put_custom_insurance**](ReferenceEndpointsApi.md#put_custom_insurance) | **PUT** /custom/insurances/{insurance_uuid} | putCustomInsurance
[**put_custom_location_type**](ReferenceEndpointsApi.md#put_custom_location_type) | **PUT** /custom/location_types/{location_type_uuid} | putCustomLocationType
[**put_custom_provider_type**](ReferenceEndpointsApi.md#put_custom_provider_type) | **PUT** /custom/provider_types/{provider_type_uuid} | putCustomProviderType
[**put_custom_specialty**](ReferenceEndpointsApi.md#put_custom_specialty) | **PUT** /custom/specialties/{specialty_uuid} | putCustomSpecialty



## delete_custom_insurance

> delete_custom_insurance(insurance_uuid)
deleteCustomInsurance

Delete an insurance.  Note: If you've added this insurance to doctors, you are deleting all instances of this UUID, and Ribbon will not be able to regenerate them. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insurance_uuid** | **uuid::Uuid** | The UUID of the target insurance. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_location_type

> delete_custom_location_type(location_type_uuid)
deleteCustomLocationType

Delete a location type.  Note: You cannot edit a Ribbon created location type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_type_uuid** | **uuid::Uuid** | The UUID of the target location type. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_provider_type

> delete_custom_provider_type(provider_type_uuid)
deleteCustomProviderType

Delete a provider type.  Note: You cannot edit a Ribbon created provider type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_type_uuid** | **uuid::Uuid** | The UUID of the target provider type. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_specialty

> delete_custom_specialty(specialty_uuid)
deleteCustomSpecialty

Delete a specialty.  Note: You cannot delete a Ribbon created specialty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**specialty_uuid** | **uuid::Uuid** | The UUID of the target specialty. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_insurance

> models::GetCustomProviders200ResponseDataInnerInsurancesInner get_custom_insurance(insurance_uuid)
getCustomInsurance

Retrieve data on a specific insurance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insurance_uuid** | **uuid::Uuid** | The UUID of the target insurance. | [required] |

### Return type

[**models::GetCustomProviders200ResponseDataInnerInsurancesInner**](getCustomProviders_200_response_data_inner_insurances_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_location_type

> models::GetCustomLocationTypes200ResponseResultsInner get_custom_location_type(location_type_uuid)
getCustomLocationType

Retrieve data on a specific location type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_type_uuid** | **uuid::Uuid** | The UUID of the target location type. | [required] |

### Return type

[**models::GetCustomLocationTypes200ResponseResultsInner**](getCustomLocationTypes_200_response_results_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_location_types

> models::GetCustomLocationTypes200Response get_custom_location_types(search)
getCustomLocationTypes

Search and list location types that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search parameter for reference endpoints (Provider Type, Location Type, Languages) |  |

### Return type

[**models::GetCustomLocationTypes200Response**](getCustomLocationTypes_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_provider_type

> models::GetCustomProviderTypes200ResponseResultsInner get_custom_provider_type(provider_type_uuid)
getCustomProviderType

Retrieve data on a specific provider type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_type_uuid** | **uuid::Uuid** | The UUID of the target provider type. | [required] |

### Return type

[**models::GetCustomProviderTypes200ResponseResultsInner**](getCustomProviderTypes_200_response_results_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_provider_types

> models::GetCustomProviderTypes200Response get_custom_provider_types(search)
getCustomProviderTypes

Search and list provider types that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search parameter for reference endpoints (Provider Type, Location Type, Languages) |  |

### Return type

[**models::GetCustomProviderTypes200Response**](getCustomProviderTypes_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_specialty

> models::GetCustomProviders200ResponseParametersSpecialty get_custom_specialty(specialty_uuid)
getCustomSpecialty

Retrieve data on a specific specialty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**specialty_uuid** | **uuid::Uuid** | The UUID of the target specialty. | [required] |

### Return type

[**models::GetCustomProviders200ResponseParametersSpecialty**](getCustomProviders_200_response_parameters_specialty.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_insurances

> models::GetInsurances200Response get_insurances(search, carrier_association, carrier_brand, carrier_name, state, plan_name, plan_type, display_name, category, _excl_category, codes, partial_codes)
getInsurances

Search and list insurances that exist within the Ribbon API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | String input that fuzzy searches across `display_name`, `carrier_name`, and `uuid`. |  |
**carrier_association** | Option<**String**> | Comma separated list of the carrier association of insurances you are searching for.  Note: This input must be an exact string match to work |  |
**carrier_brand** | Option<**String**> | Comma separated list of the carrier brand of insurances you are searching for.  Note: This input must be an exact string match to work |  |
**carrier_name** | Option<**String**> | Comma separated list of the carrier name of insurances you are searching for.  Note: This input must be an exact string match to work |  |
**state** | Option<**String**> | Two letter abbreviated state code of insurances you are searching for. |  |
**plan_name** | Option<**String**> | Exact string input of the plan name of insurances you are searching for. |  |
**plan_type** | Option<**String**> | Exact string input of the plan type of insurances you are searching for. |  |
**display_name** | Option<**String**> | Exact string input of the display name of insurances you are searching for. |  |
**category** | Option<**String**> | Comma separated list of the category of insurances you are searching for.  Note: This input must be an exact string match to work |  |
**_excl_category** | Option<**String**> | Comma separated list of the category of insurances you wish to exclude.  Note: This input must be an exact string match to work |  |
**codes** | Option<**String**> | Single code input to search for plans with an exact string match within the codes field. |  |
**partial_codes** | Option<**String**> | Partial string input to match to the codes field. For Medicare Advantage plans this is a contract ID (i.e. H9572). For Federal or State Exchange plans this is the first 10 digits of the HIOS ID (i.e. 36096il100)  Note: This parameter can only be used if the `category` param is also utilized with a single category value. |  |

### Return type

[**models::GetInsurances200Response**](getInsurances_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages

> models::GetLanguages200Response get_languages(search)
getLanguages

Search and list provider languages that exist in the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search parameter for reference endpoints (Provider Type, Location Type, Languages) |  |

### Return type

[**models::GetLanguages200Response**](getLanguages_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_procedure

> models::GetCustomProviders200ResponseParametersProcedure get_procedure(procedure_uuid)
getProcedure

Retrieve data on a specific procedure. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**procedure_uuid** | **uuid::Uuid** | The UUID of the target procedure. | [required] |

### Return type

[**models::GetCustomProviders200ResponseParametersProcedure**](getCustomProviders_200_response_parameters_procedure.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_procedures

> models::GetProcedures200Response get_procedures(search, procedure_code, page, page_size)
getProcedures

Search and list procedures that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search parameter for reference endpoints (Provider Type, Location Type, Languages) |  |
**procedure_code** | Option<**String**> | A specific billing code (e.g., CPT, DRG) to search for |  |
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |

### Return type

[**models::GetProcedures200Response**](getProcedures_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specialties

> models::GetSpecialties200Response get_specialties(page, page_size, search, provider_type)
getSpecialties

Search and list specialties that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |
**search** | Option<**String**> | String input that fuzzy searches against key fields within each specialties object to return the most relevant options. |  |
**provider_type** | Option<**String**> | 'Type' of provider specialty to filter results on. Here are a few key provider types: - Doctor - Nursing - Dental Providers |  |

### Return type

[**models::GetSpecialties200Response**](getSpecialties_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_insurance

> models::PostCustomInsurance201Response post_custom_insurance(post_custom_insurance_request)
postCustomInsurance

Create a insurance with desired field values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_custom_insurance_request** | [**PostCustomInsuranceRequest**](PostCustomInsuranceRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomInsurance201Response**](postCustomInsurance_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_location_type

> models::PostCustomLocationType201Response post_custom_location_type(post_custom_provider_type_request)
postCustomLocationType

Create a location type with desired field values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_custom_provider_type_request** | [**PostCustomProviderTypeRequest**](PostCustomProviderTypeRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomLocationType201Response**](postCustomLocationType_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_provider_type

> models::PostCustomProviderType201Response post_custom_provider_type(post_custom_provider_type_request)
postCustomProviderType

Create a custom provider type with desired field values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_custom_provider_type_request** | [**PostCustomProviderTypeRequest**](PostCustomProviderTypeRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomProviderType201Response**](postCustomProviderType_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_specialty

> models::PostCustomSpecialty201Response post_custom_specialty(post_custom_specialty_request)
postCustomSpecialty

Create a custom specialty with desired field values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_custom_specialty_request** | [**PostCustomSpecialtyRequest**](PostCustomSpecialtyRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomSpecialty201Response**](postCustomSpecialty_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_insurance

> models::PostCustomInsurance201Response put_custom_insurance(insurance_uuid, post_custom_insurance_request)
putCustomInsurance

Edit fields of a custom created insurance or a Ribbon created insurance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insurance_uuid** | **uuid::Uuid** | The UUID of the target insurance. | [required] |
**post_custom_insurance_request** | [**PostCustomInsuranceRequest**](PostCustomInsuranceRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomInsurance201Response**](postCustomInsurance_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_location_type

> models::PostCustomLocationType201Response put_custom_location_type(location_type_uuid, post_custom_provider_type_request)
putCustomLocationType

Edit fields of a custom created location type.  Note: You cannot edit a Ribbon created location type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_type_uuid** | **uuid::Uuid** | The UUID of the target location type. | [required] |
**post_custom_provider_type_request** | [**PostCustomProviderTypeRequest**](PostCustomProviderTypeRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomLocationType201Response**](postCustomLocationType_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_provider_type

> models::PostCustomProviderType201Response put_custom_provider_type(provider_type_uuid, post_custom_provider_type_request)
putCustomProviderType

Edit fields of a custom created provider type.  Note: You cannot edit a Ribbon created provider type. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_type_uuid** | **uuid::Uuid** | The UUID of the target provider type. | [required] |
**post_custom_provider_type_request** | [**PostCustomProviderTypeRequest**](PostCustomProviderTypeRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomProviderType201Response**](postCustomProviderType_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_specialty

> models::PostCustomSpecialty201Response put_custom_specialty(specialty_uuid, post_custom_specialty_request)
putCustomSpecialty

Edit fields of a custom created specialty.  Note: You cannot edit a Ribbon created specialty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**specialty_uuid** | **uuid::Uuid** | The UUID of the target specialty. | [required] |
**post_custom_specialty_request** | [**PostCustomSpecialtyRequest**](PostCustomSpecialtyRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PostCustomSpecialty201Response**](postCustomSpecialty_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

