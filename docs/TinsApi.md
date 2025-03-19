# \TinsApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_custom_tin**](TinsApi.md#get_custom_tin) | **GET** /custom/tin/{tin_id} | getCustomTin
[**get_tins**](TinsApi.md#get_tins) | **GET** /custom/tin | getTins



## get_custom_tin

> models::GetTins200ResponseTinsInner get_custom_tin(tin_id)
getCustomTin

Retrieve data on a specific TIN. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tin_id** | **uuid::Uuid** | The target TIN. | [required] |

### Return type

[**models::GetTins200ResponseTinsInner**](getTins_200_response_tins_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tins

> models::GetTins200Response get_tins(search, name, legal_name, tin_ids, has_tin, page, page_size)
getTins

Search and list tins that exist within the Ribbon API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | String input that fuzzy searches across tins, name, address, and legal_name. |  |
**name** | Option<**String**> | The billing entity name that appears on claims data, or if available, the official legal name of the billing entity. String input that is fuzzy matched against the `name` field. Note: This parameter will not match with the `legal_name` field, only the `name` field. |  |
**legal_name** | Option<**String**> | The legal name of the entity associated with the TIN. String input that is fuzzy matched against the `legal_name` field. |  |
**tin_ids** | Option<**String**> | Comma separated list of TINs. Note: This parameter cannot be used in combination with any other parameters. |  |
**has_tin** | Option<**bool**> | Boolean input that applies to tin_confirmed field. |  |
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |

### Return type

[**models::GetTins200Response**](getTins_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

