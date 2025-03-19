# \OrganizationsApi

All URIs are relative to *https://api.ribbonhealth.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /custom/organizations/{organization_uuid} | getOrganization
[**get_organizations**](OrganizationsApi.md#get_organizations) | **GET** /custom/organizations | getOrganizations



## get_organization

> models::GetOrganizations200ResponseDataInner get_organization(organization_uuid)
getOrganization

Retrieve detailed information for any organization given its UUID 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_uuid** | **uuid::Uuid** | The UUID of the target organization. | [required] |

### Return type

[**models::GetOrganizations200ResponseDataInner**](getOrganizations_200_response_data_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations

> models::GetOrganizations200Response get_organizations(address, name, distance, location, page, page_size)
getOrganizations

Search for different organizations.  #### Example Use Case Display a map of all of all nearby health systems and their relevant information so that a patient can find care. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | Option<**String**> | String input of an address that will be interpreted and geocoded in real time. |  |
**name** | Option<**String**> | String input that is fuzzy matched to against the `organization.name` field. |  |
**distance** | Option<**i32**> | Constrains the search to within this many miles of the center point |  |
**location** | Option<**String**> | Latitude/longitude pair of coordinates in lieu of a string address. |  |
**page** | Option<**i32**> | The page of the results which was returned. |  |
**page_size** | Option<**i32**> | How many results are in each page. |  |

### Return type

[**models::GetOrganizations200Response**](getOrganizations_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

