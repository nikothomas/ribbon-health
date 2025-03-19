# EditCustomLocationFilter200ResponseDataFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parameter** | **String** | The name of filter which will be used when applying to a search | 
**field** | **String** | The name of the field that the filter will be applied to  You can specify nested fields by placing a `.` between each level within the JSON object. For example, `address_details.state` | 
**value_type** | **String** | The data type of the value passed into this filter | 
**filter_type** | **String** | The type of comparison that will occur between the value passed into this filter and the field specified in the `field` parameter  Note that `boost` filters have several limitations: - A Boost Filter cannot use the following `value_type`s: `float` - If a Boost Filter's `field` targets something that is within a list of objects, such as ‘educations.education.name’, we will not reorder the list to bring these items to the front. We will merely boost records where there is an entry in the list that matches the filter, wherever it is.   - There is one exception to this: `locations`. If you search for a provider and boost fields nested within the `locations` list, we will reorder the locations to put matching locations first. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


