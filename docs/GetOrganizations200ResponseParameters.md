# GetOrganizations200ResponseParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of the results which was returned. | [optional]
**page_size** | Option<**i32**> | How many results are in each page. | [optional]
**total_count** | Option<**i32**> | The total number of results matched, across all pages. | [optional]
**sort_by** | Option<**String**> | The main criteria used to sort results in the record set. | [optional]
**distance** | Option<**i32**> | The proximity radius of locations returned. | [optional]
**geo** | Option<[**models::GetCustomProviders200ResponseParametersGeo**](getCustomProviders_200_response_parameters_geo.md)> |  | [optional]
**address** | Option<**String**> | String input of an address that will be interpreted and geocoded in real time. | [optional]
**name** | Option<**String**> | String input that is fuzzy matched to against the `organization.name` field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


