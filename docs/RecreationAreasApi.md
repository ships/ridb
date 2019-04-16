# \RecreationAreasApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organization_rec_area**](RecreationAreasApi.md#get_organization_rec_area) | **get** /organizations/{orgId}/recareas/{recAreaId} | retrieve a specific recreation area by id for an organization
[**get_organization_rec_areas**](RecreationAreasApi.md#get_organization_rec_areas) | **get** /organizations/{orgId}/recareas | retrieve all recreation areas for an organization
[**get_rec_area**](RecreationAreasApi.md#get_rec_area) | **get** /recareas/{recAreaId} | retrieve a specific recreation area by id
[**get_rec_areas**](RecreationAreasApi.md#get_rec_areas) | **get** /recareas | retrieve all recreation areas


# **get_organization_rec_area**
> ::models::RecreationArea get_organization_rec_area(ctx, org_id, rec_area_id, optional)
retrieve a specific recreation area by id for an organization

This endpoint retrieves a specific recreation area belonging to a specific organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **org_id** | **String**| id of the organization | 
  **rec_area_id** | **String**| id of the recreation area | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **org_id** | **String**| id of the organization | 
 **rec_area_id** | **String**| id of the recreation area | 
 **full** | **String**| return the full record details or compact (abbreviated) details | 

### Return type

[**::models::RecreationArea**](Recreation Area.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_organization_rec_areas**
> ::models::InlineResponse2001 get_organization_rec_areas(ctx, org_id, optional)
retrieve all recreation areas for an organization

This endpoint retrieves all recreation areas belonging to a specific organization.

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

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area**
> ::models::RecreationArea get_rec_area(ctx, rec_area_id, optional)
retrieve a specific recreation area by id

This endpoint retrieves a specific recreation area.

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
 **full** | **String**| return the full record details or compact (abbreviated) details | 

### Return type

[**::models::RecreationArea**](Recreation Area.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_areas**
> ::models::InlineResponse2001 get_rec_areas(ctx, optional)
retrieve all recreation areas

This endpoint retrieves all recreation areas.

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
 **activity** | [**Vec<String>**](String.md)| comma delimited list of activity IDs or keyword | 
 **latitude** | **f64**| Latitude of the point in decimal degrees | 
 **longitude** | **f64**| Longitude of the point in decimal degrees | 
 **radius** | **f64**| Distance (in miles) by which to include search results | [default to 25]
 **lastupdated** | **String**| return all records modified since this date **Date Format: (mm-dd-yyyy)** | 
 **sort** | **String**| sort the results by \"ID\", \"Date\" or \"Name\" | 

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

