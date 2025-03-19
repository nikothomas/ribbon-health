# GetPricingProviders200ResponseDataInnerMatchedLocationCostsInpatient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min** | **f64** | The minimum cost for this procedure in this place of service, in dollars. | 
**median** | **f64** | The median cost for this procedure in this place of service, in dollars. | 
**max** | **f64** | The maximum cost for this procedure in this place of service, in dollars. | 
**is_bundle** | **bool** | Where this data is the costs for a procedure bundle rather than a regular procedure. | 
**components** | Option<[**Vec<models::GetPricingProviders200ResponseDataInnerMatchedLocationCostsOutpatientComponentsInner>**](getPricingProviders_200_response_data_inner_matched_location_costs_outpatient_components_inner.md)> | Individual costs for the components that make up this procedure bundle.  For a regular procedure, this field will always be `null`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


