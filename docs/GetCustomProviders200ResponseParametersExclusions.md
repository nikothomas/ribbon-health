# GetCustomProviders200ResponseParametersExclusions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_types** | Option<**Vec<String>**> | List of 'types' of providers excluded. Excludes any providers with a matching provider type. | [optional]
**insurance_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of insurance uuids excluded. Excludes any providers who accept a given insurance(s). | [optional]
**location_insurance_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of insurance uuids excluded. Excludes any provider locations that accept a given insurance(s). | [optional]
**location_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of practice location uuids to exclude. Excludes providers who see patients at any of the given practice locations. | [optional]
**location_organization_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of organization uuids to exclude. Excludes providers who have the given organization uuid(s) listed in the `provider.organizations` field. | [optional]
**specialty_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list of specialty IDs excluded from the search. This can be the result of using `_excl_specialty_ids` or exclusions based on the fuzzy-matched `specialty` parameter. | [optional]
**specialty_ids_primary** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list of primary specialty IDs excluded from the search. This can be the result of using `_excl_specialty_ids_primary` or exclusions based on the fuzzy-matched `specialty_primary` parameter. | [optional]
**procedure_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Comma separated list of procedure uuids to exclude. Exclude providers who perform the given procedure. | [optional]
**language** | Option<[**models::GetCustomProviders200ResponseParametersExclusionsLanguage**](getCustomProviders_200_response_parameters_exclusions_language.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


