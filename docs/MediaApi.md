# \MediaApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_facility_media**](MediaApi.md#get_all_facility_media) | **get** /facilities/{facilityId}/media | retrieve all media for a facility
[**get_all_media**](MediaApi.md#get_all_media) | **get** /media | retrieve all media
[**get_all_rec_area_media**](MediaApi.md#get_all_rec_area_media) | **get** /recareas/{recAreaId}/media | retrieve all media for a recreation area
[**get_facility_media**](MediaApi.md#get_facility_media) | **get** /facilities/{facilityId}/media/{mediaId} | retrieve a specific media by id for a facility
[**get_media**](MediaApi.md#get_media) | **get** /media/{mediaId} | retrieve a specific media by id
[**get_rec_area_media**](MediaApi.md#get_rec_area_media) | **get** /recareas/{recAreaId}/media/{mediaId} | retrieve a specific media by id for a recreation area


# **get_all_facility_media**
> ::models::InlineResponse20013 get_all_facility_media(ctx, facility_id, optional)
retrieve all media for a facility

This endpoint retrieves all media for a specific facility.

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

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_all_media**
> ::models::InlineResponse20013 get_all_media(ctx, optional)
retrieve all media

This endpoint retrieves all media.

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

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_all_rec_area_media**
> ::models::InlineResponse20013 get_all_rec_area_media(ctx, rec_area_id, optional)
retrieve all media for a recreation area

This endpoint retrieves all media for a specific recreation area.

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

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_media**
> ::models::Media get_facility_media(ctx, facility_id, media_id)
retrieve a specific media by id for a facility

This endpoint retrieves a specific media for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **media_id** | **String**| id of the media | 

### Return type

[**::models::Media**](Media.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_media**
> ::models::Media get_media(ctx, media_id)
retrieve a specific media by id

This endpoint retrieves a specific media.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **media_id** | **String**| id of the media | 

### Return type

[**::models::Media**](Media.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_media**
> ::models::Media get_rec_area_media(ctx, rec_area_id, media_id)
retrieve a specific media by id for a recreation area

This endpoint retrieves a specific media for a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 
  **media_id** | **String**| id of the media | 

### Return type

[**::models::Media**](Media.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

