# \NetworksApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_network_analysis**](NetworksApi.md#get_network_analysis) | **GET** /network_analysis | getNetworkAnalysis



## get_network_analysis

> models::GetNetworkAnalysis200Response get_network_analysis(insurance_id, ssa_codes, exclude_npis)
getNetworkAnalysis

View a provider network across different geographies (i.e. counties).  #### Example Use Case In looking to expand to a new region, analyze existing provider networks in the region to understand how best to construct your own. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insurance_id** | **uuid::Uuid** | A unique identifier for a single provider network from the Insurances reference endpoint. | [required] |
**ssa_codes** | **String** | SSA codes of the counties to run a network analysis on.   A maximum of 50 codes may be included in a single request. | [required] |
**exclude_npis** | Option<**bool**> | When set to `true` the response will not list specific NPIs in the network but will continue to include the `npi_count` fields to let you know how many there are.  This parameter can be used to greater lower the amount of data sent back when specific NPIs are not necessary. |  |[default to false]

### Return type

[**models::GetNetworkAnalysis200Response**](getNetworkAnalysis_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

