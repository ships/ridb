# \FacilitiesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_facilities**](FacilitiesApi.md#get_facilities) | **get** /facilities | retrieve all facilities
[**get_facility**](FacilitiesApi.md#get_facility) | **get** /facilities/{facilityId} | retrieve a specific facility by id
[**get_organization_facilities**](FacilitiesApi.md#get_organization_facilities) | **get** /organizations/{orgId}/facilities | retrieve all facilities for an organization
[**get_organization_facility**](FacilitiesApi.md#get_organization_facility) | **get** /organizations/{orgId}/facilities/{facilityId} | retrieve a specific facility by id for an organization
[**get_rec_area_facilities**](FacilitiesApi.md#get_rec_area_facilities) | **get** /recareas/{recAreaId}/facilities | retrieve all facilities for a recreation area
[**get_rec_area_facility**](FacilitiesApi.md#get_rec_area_facility) | **get** /recareas/{recAreaId}/facilities/{facilityId} | retrieve a specific facility by id for a recreation area


# **get_facilities**
> ::models::InlineResponse2002 get_facilities(ctx, optional)
retrieve all facilities

This endpoint retrieves all facilities

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]
 **full** | **String**| return the full record details or compact (abbreviated) details | 
 **state** | [**Vec<String>**](String.md)| comma delimited list of 2 character state codes | 
 **latitude** | **f64**| Latitude of the point in decimal degrees | 
 **longitude** | **f64**| Longitude of the point in decimal degrees | 
 **radius** | **f64**| Distance (in miles) by which to include search results | [default to 25]
 **activity** | [**Vec<String>**](String.md)| comma delimited list of activity IDs or keyword | 
 **lastupdated** | **String**| return all records modified since this date **Date Format: (mm-dd-yyyy)** | 
 **sort** | **String**| sort the results by \"ID\", \"Date\" or \"Name\" | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility**
> ::models::Facility get_facility(ctx, facility_id, optional)
retrieve a specific facility by id

This endpoint retrieves a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **facility_id** | **String**| id of the facility | 
 **full** | **String**| return the full record details or compact (abbreviated) details | 

### Return type

[**::models::Facility**](Facility.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_organization_facilities**
> ::models::InlineResponse2002 get_organization_facilities(ctx, org_id, optional)
retrieve all facilities for an organization

This endpoint retrieves all facilities belonging to a specific organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **org_id** | **String**| id of the organization | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **org_id** | **String**| id of the organization | 
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]
 **full** | **String**| return the full record details or compact (abbreviated) details | 
 **state** | [**Vec<String>**](String.md)| comma delimited list of 2 character state codes | 
 **activity** | [**Vec<String>**](String.md)| comma delimited list of activity IDs or keyword | 
 **lastupdated** | **String**| return all records modified since this date **Date Format: (mm-dd-yyyy)** | 
 **sort** | **String**| sort the results by \"ID\", \"Date\" or \"Name\" | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_organization_facility**
> ::models::Facility get_organization_facility(ctx, org_id, facility_id, optional)
retrieve a specific facility by id for an organization

This endpoint retrieves a specific facility belonging to a specific organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **org_id** | **String**| id of the organization | 
  **facility_id** | **String**| id of the facility | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **org_id** | **String**| id of the organization | 
 **facility_id** | **String**| id of the facility | 
 **full** | **String**| return the full record details or compact (abbreviated) details | 

### Return type

[**::models::Facility**](Facility.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_facilities**
> ::models::InlineResponse2002 get_rec_area_facilities(ctx, rec_area_id, optional)
retrieve all facilities for a recreation area

This endpoint retrieves all facilities belonging to a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rec_area_id** | **String**| id of the recreation area | 
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]
 **full** | **String**| return the full record details or compact (abbreviated) details | 
 **state** | [**Vec<String>**](String.md)| comma delimited list of 2 character state codes | 
 **activity** | [**Vec<String>**](String.md)| comma delimited list of activity IDs or keyword | 
 **lastupdated** | **String**| return all records modified since this date **Date Format: (mm-dd-yyyy)** | 
 **sort** | **String**| sort the results by \"ID\", \"Date\" or \"Name\" | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_facility**
> ::models::Facility get_rec_area_facility(ctx, rec_area_id, facility_id, optional)
retrieve a specific facility by id for a recreation area

This endpoint retrieves a specific facility belonging to a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 
  **facility_id** | **String**| id of the facility | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rec_area_id** | **String**| id of the recreation area | 
 **facility_id** | **String**| id of the facility | 
 **full** | **String**| return the full record details or compact (abbreviated) details | 

### Return type

[**::models::Facility**](Facility.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

