# GetCustomLocations200ResponseParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_count** | Option<**i32**> | The total number of results matched, across all pages. | [optional]
**sort_by** | Option<**String**> | The main criteria used to sort results in the record set. | [optional]
**geo** | Option<[**models::GetCustomProviders200ResponseParametersGeo**](getCustomProviders_200_response_parameters_geo.md)> |  | [optional]
**page** | Option<**i32**> | The page of the results which was returned. | [optional]
**page_size** | Option<**i32**> | How many results are in each page. | [optional]
**fields** | Option<**Vec<String>**> | List of fields within the location object to return. Can be used to greatly reduce the size of the response by requesting only data you intend to use.  Cannot be used in tandem with `_excl_fields` | [optional]
**_excl_fields** | Option<**Vec<String>**> | List of fields within the location object to exclude from the response. Can be used to greatly reduce the size of the response by requesting only data you intend to use. | [optional]
**address** | Option<**String**> | String input of an address that will be interpreted and geocoded in real time. | [optional]
**name** | Option<**String**> | String input for a fuzzy search on location name. | [optional]
**distance** | Option<**i32**> | The proximity radius of locations returned. | [optional]
**location_types** | Option<**Vec<String>**> | List of values that filter to what type of facilities to show. We categorize locations into 34 types. Clients can add their own locations with unique location_types, and they will also be included in the search. See the Location Types Reference Endpoint for a list of all location types. | [optional]
**_excl_location_types** | Option<**Vec<String>**> | List of the 'types' of locations to exclude. Excludes locations with a matching location type. | [optional]
**insurance_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of desired insurance uuids. See all locations that accept a given insurance(s). | [optional]
**_excl_insurance_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of insurance uuids to exclude. Exclude locations that accept a given insurance(s). | [optional]
**insurance_carrier_name** | Option<**String**> | String input of carrier_name in order to search for all locations that take at least one plan from a given insurance carrier. Find the individual valid carrier_name values from the insurance objects returned in the Insurances Reference Endpoint. Note: This input must be an exact string match to work | [optional]
**min_confidence** | Option<**u32**> | Integer input (0-5) of the minimum confidence threshold for returned locations. min_location_confidence=4 will only display locations that have a confidence 4 or higher. | [optional]
**national_bluecard** | Option<**bool**> | Boolean input that enables an API search to automatically default to the National BlueCard EPO/PPO Network whenever a member searches for out-of-state, in-network care and is covered by a BCBS Association PPO insurance plan. Use the parameter in conjunction with the address parameter and either the insurance_ids or insurance fuzzy search parameters. Defaults to true unless otherwise specified. | [optional]
**organization_ids** | Option<**Vec<String>**> | Comma separated list of desired organization uuids. Filters to only locations that are affiliated with the given organization uuid(s). | [optional]
**_excl_organization_ids** | Option<**Vec<String>**> | Comma separated list of organization uuids to exclude. Excludes locations that are affiliated with the given organization uuid(s). | [optional]
**clinical_area** | Option<[**models::GetCustomProviders200ResponseParametersClinicalAreaIds**](getCustomProviders_200_response_parameters_clinical_area_ids.md)> |  | [optional]
**clinical_area_ids** | Option<**Vec<String>**> | List of desired clinical area ids. Returns all locations with a clinical area exactly matching any of the entered IDs. (Note: Use the `/clinical_areas/` reference endpoint to identify relevant IDs) | [optional]
**treatment** | Option<[**models::GetCustomProviders200ResponseParametersClinicalAreaIdsTreatmentsInner**](getCustomProviders_200_response_parameters_clinical_area_ids_treatments_inner.md)> |  | [optional]
**treatment_ids** | Option<**Vec<String>**> | List of desired treatment ids. Returns all providers with a `treatments.uuid` field exactly matching any of the entered IDs. (Note: Use the /treatments/ reference endpoint (docs) to identify relevant IDs) | [optional]
**tin_ids** | Option<**String**> | List of desired TINs. | [optional]
**tin_name** | Option<**String**> |  | [optional]
**tin_legal_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


