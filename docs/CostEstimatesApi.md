# \CostEstimatesApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_eligibility**](CostEstimatesApi.md#get_eligibility) | **POST** /eligibility | getEligibility
[**get_eligibility_insurance_partner**](CostEstimatesApi.md#get_eligibility_insurance_partner) | **GET** /eligibility_insurance_partners/{insurance_partner} | getEligibilityInsurancePartner
[**get_eligibility_insurance_partners**](CostEstimatesApi.md#get_eligibility_insurance_partners) | **GET** /eligibility_insurance_partners | getEligibilityInsurancePartners
[**get_procedure_cost_estimate**](CostEstimatesApi.md#get_procedure_cost_estimate) | **GET** /procedure_cost_estimate | getProcedureCostEstimate



## get_eligibility

> models::GetEligibility200Response get_eligibility(get_eligibility_request, include_full_response)
getEligibility

Verify a member's current insurance coverage and benefits. You can access detailed information including a member’s progress on their Deductible and Out-of-pocket, as well as Copay and Coinsurance information for different services.  #### Example Use Case Inform a member of their current progress against their Deductible as well as their Copay and Coinsurance summaries so they can better estimate their out-of pocket costs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eligibility_request** | [**GetEligibilityRequest**](GetEligibilityRequest.md) | Identifying information for the member to be checked | [required] |
**include_full_response** | Option<**bool**> | Whether to return the full eligibility response from the payer. Defaults to `false` when not provided. |  |[default to false]

### Return type

[**models::GetEligibility200Response**](getEligibility_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eligibility_insurance_partner

> models::GetEligibilityInsurancePartners200ResponseDataInner get_eligibility_insurance_partner(insurance_partner)
getEligibilityInsurancePartner

Fetch an insurance partner supported by our eligibility features. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insurance_partner** | **String** | The identifier for this insurance partner in the Ribbon API | [required] |

### Return type

[**models::GetEligibilityInsurancePartners200ResponseDataInner**](getEligibilityInsurancePartners_200_response_data_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eligibility_insurance_partners

> models::GetEligibilityInsurancePartners200Response get_eligibility_insurance_partners(search)
getEligibilityInsurancePartners

Search or list all insurance partners supported by our eligibility features. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Fuzzy search input for insurance names |  |

### Return type

[**models::GetEligibilityInsurancePartners200Response**](getEligibilityInsurancePartners_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_procedure_cost_estimate

> models::GetProcedureCostEstimate200Response get_procedure_cost_estimate(procedure_ids, member_zip, r#type, plan_id)
getProcedureCostEstimate

Calculates estimated costs for a given procedure based on a user's location.  #### Example Use Case Estimate the cost of a knee replacement surgery for a user in Boston so they can plan their personal finances accordingly. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**procedure_ids** | **String** | The UUIDs of the procedures to filter results to, as a comma-delimited string | [required] |
**member_zip** | **String** | The zip code of the member we are generating cost estimates for | [required] |
**r#type** | Option<**String**> | The type of data to base cost estimates on: `claims` data or payer-filed `price_transparency` data. Defaults to `claims`. |  |
**plan_id** | Option<**uuid::Uuid**> | The UUID of the insurance plan to use when fetching negotiated rates with providers. Necessary only if searching for estimates using the 'price_transparency' `type`. |  |

### Return type

[**models::GetProcedureCostEstimate200Response**](getProcedureCostEstimate_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

