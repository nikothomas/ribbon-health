# GetTins200ResponseTinsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tin** | Option<**String**> | Standard 9-digit identification code used by the IRS for business entities and used for contracting and paying provider/facility claims. | [optional]
**name** | Option<**String**> | The billing entity name that appears on claims data, or if available, the official legal name of the billing entity. | [optional]
**legal_name** | Option<**String**> | The legal name of the entity associated with the TIN. | [optional]
**address** | Option<**String**> | The address of the organization with the TIN. This could be the primary service location or billing location. | [optional]
**tin_confirmed** | Option<**bool**> | A yes/no field that assesses whether a TIN is likely to be valid or not. The field is powered by business logic that triangulates IRS data and claims data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


