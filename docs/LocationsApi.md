# \LocationsApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_custom_location**](LocationsApi.md#delete_custom_location) | **DELETE** /custom/locations/{location_uuid} | deleteCustomLocation
[**get_custom_location**](LocationsApi.md#get_custom_location) | **GET** /custom/locations/{location_uuid} | getCustomLocation
[**get_custom_locations**](LocationsApi.md#get_custom_locations) | **GET** /custom/locations | getCustomLocations
[**post_custom_locations**](LocationsApi.md#post_custom_locations) | **POST** /custom/locations | postCustomLocations
[**put_custom_location**](LocationsApi.md#put_custom_location) | **PUT** /custom/locations/{location_uuid} | putCustomLocation
[**put_custom_location_clinical_areas**](LocationsApi.md#put_custom_location_clinical_areas) | **PUT** /custom/locations/{location_uuid}/clinical_areas | putCustomLocationClinicalAreas
[**put_custom_location_insurances**](LocationsApi.md#put_custom_location_insurances) | **PUT** /custom/locations/{location_uuid}/insurances | putCustomLocationInsurances
[**put_custom_location_organizations**](LocationsApi.md#put_custom_location_organizations) | **PUT** /custom/locations/{location_uuid}/organizations | putCustomLocationOrganizations



## delete_custom_location

> delete_custom_location(location_uuid)
deleteCustomLocation

Delete a location. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_location

> models::GetCustomProvider200ResponseLocationsInner get_custom_location(location_uuid)
getCustomLocation

Retrieve data on a specific location. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |

### Return type

[**models::GetCustomProvider200ResponseLocationsInner**](getCustomProvider_200_response_locations_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_locations

> models::GetCustomLocations200Response get_custom_locations(page, page_size, fields, _excl_fields, address, name, distance, location_types, _excl_location_types, location, location_ids, _excl_location_ids, insurance_ids, _excl_insurance_ids, insurance_carrier_name, min_confidence, national_bluecard, organization_ids, _excl_organization_ids, clinical_area, clinical_area_ids, _excl_clinical_area_ids, treatment, treatment_ids, _excl_treatment_ids, tin_ids, tin_name, tin_legal_name)
getCustomLocations

Allows you to search for different service locations, including specific location types. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |
**fields** | Option<**String**> | Comma separated list of fields within the location object to return. Can be used to greatly reduce the size of the response by requesting only data you intend to use. Cannot be used in tandem with `_excl_fields` |  |
**_excl_fields** | Option<**String**> | Comma separated list of fields within the location object to exclude from the response. Can be used to greatly reduce the size of the response by requesting only data you intend to use. |  |
**address** | Option<**String**> | String input of an address that will be interpreted and geocoded in real time. |  |
**name** | Option<**String**> | String input for a 'fuzzy' search on location name. |  |
**distance** | Option<**i32**> | The proximity radius of locations returned. |  |
**location_types** | Option<**String**> | Comma separated list of values that filter to what type of facilities to show. We categorize locations into 36 types. Clients can add their own locations with unique location_types, and they will also be included in the search. See the Location Types Reference Endpoint for a list of all location types. |  |
**_excl_location_types** | Option<**String**> | Comma separated list of the 'types' of locations to exclude. Excludes locations with a matching location type. |  |
**location** | Option<**String**> | Latitude/longitude pair of coordinates in lieu of a string address. |  |
**location_ids** | Option<**String**> | Comma separated list of desired practice location uuids. |  |
**_excl_location_ids** | Option<**String**> | Comma separated list of practice location uuids to exclude. |  |
**insurance_ids** | Option<**String**> | Comma separated list of desired insurance uuids. See all locations that accept a given insurance(s). |  |
**_excl_insurance_ids** | Option<**String**> | Comma separated list of insurance uuids to exclude. Exclude locations that accept a given insurance(s). |  |
**insurance_carrier_name** | Option<**String**> | String input of carrier_name in order to search for all locations that take at least one plan from a given insurance carrier. Find the individual valid carrier_name values from the insurance objects returned in the Insurances Reference Endpoint. Note: This input must be an exact string match to work |  |
**min_confidence** | Option<**i32**> | Integer input (0-5) of the minimum confidence threshold for returned locations. min_location_confidence=4 will only display locations that have a confidence 4 or higher. |  |
**national_bluecard** | Option<**bool**> | Boolean input that enables an API search to automatically default to the National BlueCard EPO/PPO Network whenever a member searches for out-of-state, in-network care and is covered by a BCBS Association PPO insurance plan. Use the parameter in conjunction with the address parameter and either the insurance_ids or insurance fuzzy search parameters. Defaults to true unless otherwise specified. |  |
**organization_ids** | Option<**String**> | Comma separated list of desired organization uuids. Filters to only locations that are affiliated with the given organization uuid(s). |  |
**_excl_organization_ids** | Option<**String**> | Comma separated list of organization uuids to exclude. Excludes locations that are affiliated with the given organization uuid(s). |  |
**clinical_area** | Option<**String**> | String input that is fuzzy matched to the most relevant `clinical_area.display` field. Only a single clinical area will be selected. Returns all location with this clinical area. |  |
**clinical_area_ids** | Option<**String**> | Comma-separated list of desired clinical area ids. Returns all locations with a clinical area exactly matching any of the entered IDs. (Note: Use the `/clinical_areas/` reference endpoint to identify relevant IDs) |  |
**_excl_clinical_area_ids** | Option<**String**> | Comma-separated list of clinical area ids to exclude. Returns all locations without a clinical area exactly matching any of the entered IDs. (Note: Use the `/clinical_areas/` reference endpoint to identify relevant IDs) |  |
**treatment** | Option<**String**> | String input that is fuzzy matched to the most relevant `treatments.display` field. Only a single treatment will be selected. Returns all locations with this treatment. |  |
**treatment_ids** | Option<**String**> | Comma-separated list of desired treatment ids. Returns all locations with a `treatments.uuid` field exactly matching any of the entered IDs. (Note: Use the /treatments/ reference endpoint (docs) to identify relevant IDs) |  |
**_excl_treatment_ids** | Option<**String**> | Comma-separated list of treatment ids to exclude. Returns all locations without a `treatments.uuid` field exactly matching any of the entered IDs. (Note: Use the /treatments/ reference endpoint (docs) to identify relevant IDs) |  |
**tin_ids** | Option<**String**> | Comma separated list of desired TINs. Filters to only locations that are affiliated with the given TINs. Note: This parameter cannot be used in combination with any other TINs related parameters. All other TINs related parameters will be ignored. |  |
**tin_name** | Option<**String**> | String input that is fuzzy matched against the `tins.name` field. Filters to only locations that are affiliated with the given TINs name. |  |
**tin_legal_name** | Option<**String**> | String input that is fuzzy matched against the `tins.legal_name` field. Filters to only locations that are affiliated with the given TINs legal name. |  |

### Return type

[**models::GetCustomLocations200Response**](getCustomLocations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_locations

> models::PostCustomLocations201Response post_custom_locations(post_custom_locations_request)
postCustomLocations

Create new locations and facilities.  #### Example Use Case You want to add new urgent care locations (or labs, imaging centers, therapy centers, etc.) to an area that are not yet included in the existing Ribbon locations listings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_custom_locations_request** | [**PostCustomLocationsRequest**](PostCustomLocationsRequest.md) | A JSON object describing the location you want to create. | [required] |

### Return type

[**models::PostCustomLocations201Response**](postCustomLocations_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_location

> models::PutCustomLocation200Response put_custom_location(location_uuid, put_custom_location_request)
putCustomLocation

Edit all fields that do not fall under `insurances`, `google_maps_link`, `latitude`, or `longitude`. You may also add new fields or remove existing fields. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_location_request** | [**PutCustomLocationRequest**](PutCustomLocationRequest.md) | A JSON object mapping the name of the field to update to its new value | [required] |

### Return type

[**models::PutCustomLocation200Response**](putCustomLocation_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_location_clinical_areas

> models::PutCustomLocationInsurances200Response put_custom_location_clinical_areas(location_uuid, put_custom_location_clinical_areas_request)
putCustomLocationClinicalAreas

Add or remove clinical areas from a location using our standard clinical area UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_location_clinical_areas_request** | [**PutCustomLocationClinicalAreasRequest**](PutCustomLocationClinicalAreasRequest.md) | A set of instructions for how to update the location's clinical areas. | [required] |

### Return type

[**models::PutCustomLocationInsurances200Response**](putCustomLocationInsurances_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_location_insurances

> models::PutCustomLocationInsurances200Response put_custom_location_insurances(location_uuid, put_custom_location_insurances_request)
putCustomLocationInsurances

Add or remove insurances from a location using our standard insurance UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_location_insurances_request** | [**PutCustomLocationInsurancesRequest**](PutCustomLocationInsurancesRequest.md) | A set of instructions for how to update the location's insurances. | [required] |

### Return type

[**models::PutCustomLocationInsurances200Response**](putCustomLocationInsurances_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_location_organizations

> models::PutCustomLocationInsurances200Response put_custom_location_organizations(location_uuid, put_custom_location_organizations_request)
putCustomLocationOrganizations

Add or remove organizations from a location using our standard organization UUIDs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_uuid** | **uuid::Uuid** | The UUID of the target location. | [required] |
**put_custom_location_organizations_request** | [**PutCustomLocationOrganizationsRequest**](PutCustomLocationOrganizationsRequest.md) | A set of instructions for how to update the location's organizations. | [required] |

### Return type

[**models::PutCustomLocationInsurances200Response**](putCustomLocationInsurances_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

