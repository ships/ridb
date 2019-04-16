# \LinksApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_facility_link**](LinksApi.md#get_facility_link) | **get** /facilities/{facilityId}/links/{linkId} | retrieve a specific link by id for a facility
[**get_facility_links**](LinksApi.md#get_facility_links) | **get** /facilities/{facilityId}/links | retrieve all links for a facility
[**get_link**](LinksApi.md#get_link) | **get** /links/{linkId} | retrieve a specific link by id
[**get_links**](LinksApi.md#get_links) | **get** /links | retrieve all links
[**get_rec_area_link**](LinksApi.md#get_rec_area_link) | **get** /recareas/{recAreaId}/links/{linkId} | retrieve a specific link by id for a recreation area
[**get_rec_area_links**](LinksApi.md#get_rec_area_links) | **get** /recareas/{recAreaId}/links | retrieve all links for a recreation area


# **get_facility_link**
> ::models::Link get_facility_link(ctx, facility_id, link_id)
retrieve a specific link by id for a facility

This endpoint retrieves a specific link for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **link_id** | **String**| id of the link | 

### Return type

[**::models::Link**](Link.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_links**
> ::models::InlineResponse20012 get_facility_links(ctx, facility_id, optional)
retrieve all links for a facility

This endpoint retrieves all links for a specific facility.

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
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_link**
> ::models::Link get_link(ctx, link_id)
retrieve a specific link by id

This endpoint retrieves a specific link.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **link_id** | **String**| id of the link | 

### Return type

[**::models::Link**](Link.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_links**
> ::models::InlineResponse20012 get_links(ctx, optional)
retrieve all links

This endpoint retrieves all links.

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

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_link**
> ::models::Link get_rec_area_link(ctx, rec_area_id, link_id)
retrieve a specific link by id for a recreation area

This endpoint retrieves a specific link for a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 
  **link_id** | **String**| id of the link | 

### Return type

[**::models::Link**](Link.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_links**
> ::models::InlineResponse20012 get_rec_area_links(ctx, rec_area_id, optional)
retrieve all links for a recreation area

This endpoint retrieves all links for a specific recreation area.

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

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

