# GetTins200ResponseParametersOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**search** | Option<**String**> | String input that fuzzy searches across TINs, names, address, and legal_name. | [optional]
**name** | Option<**String**> | The billing entity name that appears on claims data, or if available, the official legal name of the billing entity. String input that is fuzzy matched against the `name` field. | [optional]
**legal_name** | Option<**String**> | The legal name of the entity associated with the TIN. String input that is fuzzy matched against the `legal_name` field. | [optional]
**has_tin** | Option<**bool**> | Boolean input that applies to tin_confirmed field. | [optional]
**page** | Option<**i32**> | The page of the results which was returned. | [optional]
**page_size** | Option<**i32**> | The number of results per page. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


