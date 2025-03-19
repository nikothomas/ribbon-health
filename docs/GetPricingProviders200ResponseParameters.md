# GetPricingProviders200ResponseParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_count** | Option<**i32**> | The total number of results matched, across all pages. | [optional]
**page** | Option<**i32**> | The page of the results which was returned. | [optional]
**page_size** | Option<**i32**> | How many results are in each page. | [optional]
**procedure_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The UUID of the procedure that results were filtered to.  Only populated when the `procedure_id` search parameter was used. | [optional]
**procedure** | Option<[**models::GetCustomProviders200ResponseParametersProcedure**](getCustomProviders_200_response_parameters_procedure.md)> |  | [optional]
**insurances** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of insurance UUIDs for this provider. | [optional]
**insurance** | Option<[**models::GetPricingProviders200ResponseParametersInsurance**](getPricingProviders_200_response_parameters_insurance.md)> |  | [optional]
**specialty_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A comma separated list of specialty UUIDs. Filter to providers with any of the given specialties. | [optional]
**specialty** | Option<[**models::GetCustomProviders200ResponseParametersSpecialty**](getCustomProviders_200_response_parameters_specialty.md)> |  | [optional]
**address** | Option<**String**> | String input of an address that will be interpreted and geocoded in real time. | [optional]
**location** | Option<**String**> | Latitude/longitude pair of coordinates in lieu of a string address. | [optional]
**distance** | Option<**i32**> | The proximity radius of providers returned. | [optional]
**fields** | Option<**Vec<String>**> | List of fields within the provider object to return. Can be used to greatly reduce the size of the response by requesting only data you intend to use.  Note that all price information is nested under the `matched_location` field. You almost certainly want to return this field.  Cannot be used in tandem with `_excl_fields` | [optional]
**_excl_fields** | Option<**Vec<String>**> | List of fields within the provider object to exclude from the response. Can be used to greatly reduce the size of the response by requesting only data you intend to use.  Cannot be used in tandem with `fields` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


