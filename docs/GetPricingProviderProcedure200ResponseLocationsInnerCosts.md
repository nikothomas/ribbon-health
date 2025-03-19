# GetPricingProviderProcedure200ResponseLocationsInnerCosts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min** | Option<**f64**> | The minimum cost for this procedure in dollars | [optional]
**avg** | Option<**f64**> | The median cost for this procedure in dollars  This key is misnamed. For backwards compatibility, we have retained the legacy name. | [optional]
**max** | Option<**f64**> | The maximum cost for this procedure in dollars | [optional]
**is_bundle** | Option<**bool**> | Where this data is the costs for a procedure bundle rather than a regular procedure. | [optional]
**outpatient** | Option<[**models::GetPricingProviders200ResponseDataInnerMatchedLocationCostsOutpatient**](getPricingProviders_200_response_data_inner_matched_location_costs_outpatient.md)> |  | [optional]
**inpatient** | Option<[**models::GetPricingProviders200ResponseDataInnerMatchedLocationCostsInpatient**](getPricingProviders_200_response_data_inner_matched_location_costs_inpatient.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


