# GetNetworkAnalysis200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ssa_code** | **String** | The SSA code of the county this data is for. | 
**display** | **String** | The display name of the county this data is for. | 
**npi_count** | **i32** | How many unique NPIs accept the given insurance in this county. | 
**npis** | Option<**Vec<String>**> | The NPIs of providers who accept the given insurance in this county.  If the `exclude_npis` parameter was set to true, this key will not be present. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


