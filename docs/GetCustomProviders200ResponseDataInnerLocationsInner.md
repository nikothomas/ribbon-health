# GetCustomProviders200ResponseDataInnerLocationsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A UUID uniquely identifying this location | [optional]
**name** | Option<**String**> |  | [optional]
**address** | Option<**String**> |  | [optional]
**address_details** | Option<[**models::GetCustomProviders200ResponseDataInnerLocationsInnerAddressDetails**](getCustomProviders_200_response_data_inner_locations_inner_address_details.md)> |  | [optional]
**latitude** | Option<**f64**> |  | [optional]
**longitude** | Option<**f64**> |  | [optional]
**google_maps_link** | Option<**String**> |  | [optional]
**phone_numbers** | Option<[**Vec<models::GetCustomProviders200ResponseDataInnerLocationsInnerPhoneNumbersInner>**](getCustomProviders_200_response_data_inner_locations_inner_phone_numbers_inner.md)> |  | [optional]
**faxes** | Option<[**Vec<models::GetCustomProviders200ResponseDataInnerLocationsInnerFaxesInner>**](getCustomProviders_200_response_data_inner_locations_inner_faxes_inner.md)> | Fax numbers associated with this location.  This property only appears for customers purchasing fax data. If you would like this property and are not receiving it, please reach out to support. | [optional]
**confidence** | Option<**i32**> | Each location contains a confidence score. This score indicates the probability of the given provider practicing at said location with the included contact information  This field will only be populated for Ribbon-provided locations. Locations you create yourself will have a confidence score of `null`. | [optional]
**insurances** | Option<[**Vec<models::GetCustomProviders200ResponseDataInnerInsurancesInner>**](getCustomProviders_200_response_data_inner_insurances_inner.md)> | List of insurances the accepted at this location | [optional]
**tins** | Option<**String**> | Comma separated list of standard 9-digit identification code(s) used by the IRS for business entities and used for contracting and paying provider/facility claims. | [optional]
**distance** | Option<**f64**> | This location's distance from the center of a geographic search, in miles. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


