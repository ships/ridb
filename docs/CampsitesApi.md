# \CampsitesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_campsite**](CampsitesApi.md#get_campsite) | **get** /campsites/{campsiteId} | retrieve a specific campsite by id
[**get_campsites**](CampsitesApi.md#get_campsites) | **get** /campsites | retrieve all campsites
[**get_facility_campsite**](CampsitesApi.md#get_facility_campsite) | **get** /facilities/{facilityId}/campsites/{campsiteId} | retrieve a specific campsite by id for a facility
[**get_facility_campsites**](CampsitesApi.md#get_facility_campsites) | **get** /facilities/{facilityId}/campsites | retrieve all campsites for a facility


# **get_campsite**
> ::models::Campsite get_campsite(ctx, campsite_id)
retrieve a specific campsite by id

This endpoint retrieves a specific campsite.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **campsite_id** | **String**| id of the campsite | 

### Return type

[**::models::Campsite**](Campsite.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_campsites**
> ::models::InlineResponse2005 get_campsites(ctx, optional)
retrieve all campsites

This endpoint retrieves all campsites

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

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_campsite**
> ::models::Campsite get_facility_campsite(ctx, facility_id, campsite_id)
retrieve a specific campsite by id for a facility

This endpoint retrieves a specific campsite for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **campsite_id** | **String**| id of the campsite | 

### Return type

[**::models::Campsite**](Campsite.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_campsites**
> ::models::InlineResponse2005 get_facility_campsites(ctx, facility_id, optional)
retrieve all campsites for a facility

This endpoint retrieves all campsites for a specific facility.

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

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

