# GetOrganizations200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A UUID uniquely identifying this organization | [optional]
**name** | Option<**String**> |  | [optional]
**organization_types** | Option<**Vec<String>**> | The types of this organization. We currently only support `Health System` organizations, but plan to expand this in the future | [optional]
**websites** | Option<[**Vec<models::GetOrganizations200ResponseDataInnerWebsitesInner>**](getOrganizations_200_response_data_inner_websites_inner.md)> | Website(s) of this organization | [optional]
**ids** | Option<[**Vec<models::GetOrganizations200ResponseDataInnerIdsInner>**](getOrganizations_200_response_data_inner_ids_inner.md)> |  | [optional]
**address** | Option<**String**> |  | [optional]
**address_details** | Option<[**models::GetOrganizations200ResponseDataInnerAddressDetails**](getOrganizations_200_response_data_inner_address_details.md)> |  | [optional]
**latitude** | Option<**f64**> |  | [optional]
**longitude** | Option<**f64**> |  | [optional]
**phone_numbers** | Option<[**Vec<models::GetOrganizations200ResponseDataInnerPhoneNumbersInner>**](getOrganizations_200_response_data_inner_phone_numbers_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


