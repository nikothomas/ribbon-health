# GetPricingProviders200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**insurance** | Option<[**models::GetPricingProviders200ResponseDataInnerInsurance**](getPricingProviders_200_response_data_inner_insurance.md)> |  | [optional]
**procedure** | Option<[**models::GetPricingProviders200ResponseDataInnerProcedure**](getPricingProviders_200_response_data_inner_procedure.md)> |  | [optional]
**matched_location** | Option<[**models::GetPricingProviders200ResponseDataInnerMatchedLocation**](getPricingProviders_200_response_data_inner_matched_location.md)> |  | [optional]
**npi** | Option<**u32**> | The healthcare provider's 10-digit National Provider Identifier (NPI) | [optional]
**first_name** | Option<**String**> | First name of the provider | [optional]
**middle_name** | Option<**String**> | Middle name of the provider | [optional]
**last_name** | Option<**String**> | Last name of the provider | [optional]
**age** | Option<**i32**> | The estimated age of the provider | [optional]
**gender** | Option<**String**> | The gender of the provider | [optional]
**ratings_count** | Option<**i32**> | Total number of ratings collected across different sources | [optional]
**ratings_avg** | Option<**f64**> | Average patient satisfaction rating out of 10 points across multiple sources | [optional]
**degrees** | Option<**Vec<String>**> | Lists all degrees associated with this provider (e.g. MD, OD, PhD) | [optional]
**specialties** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | This lists the UUIDs of all the specialties for a given provider | [optional]
**languages** | Option<**Vec<String>**> | List of confirmed languages spoken | [optional]
**educations** | Option<[**Vec<models::GetCustomProviders200ResponseDataInnerEducationsInner>**](getCustomProviders_200_response_data_inner_educations_inner.md)> | List of the schools attended by the provider | [optional]
**insurances** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of the UUIDs of insurances the provider accepts | [optional]
**provider_types** | Option<**Vec<String>**> | There are high level classifications for different provider types -- e.g. \"Doctor\", \"Optometry\", \"Dental Providers\", \"Nursing\", etc. | [optional]
**locations** | Option<[**Vec<models::GetCustomProvider200ResponseLocationsInner>**](getCustomProvider_200_response_locations_inner.md)> | List of all locations this provider is known to practice at including any known phone numbers at these locations | [optional]
**online_profiles** | Option<[**Vec<models::GetCustomProviders200ResponseDataInnerOnlineProfilesInner>**](getCustomProviders_200_response_data_inner_online_profiles_inner.md)> | We aggregate profiles across a variety of different online sources, including booking platforms | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


