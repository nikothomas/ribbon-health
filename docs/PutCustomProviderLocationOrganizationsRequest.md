# PutCustomProviderLocationOrganizationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list of UUIDs to add to this provider's set. | [optional]
**remove** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list of UUIDs to remove to this provider's set. | [optional]
**r#override** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list of UUIDs to remove to completely replace this provider's set with. Not supported in combination with either `add` or `remove`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


