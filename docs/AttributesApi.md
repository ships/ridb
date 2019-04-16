# \AttributesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_campsite_attributes**](AttributesApi.md#get_campsite_attributes) | **get** /campsites/{campsiteId}/attributes | retrieve all attributes for a campsite
[**get_permit_entrance_attributes**](AttributesApi.md#get_permit_entrance_attributes) | **get** /permitentrances/{permitEntranceId}/attributes | retrieve all attributes for a permit entrance
[**get_tour_attributes**](AttributesApi.md#get_tour_attributes) | **get** /tours/{tourId}/attributes | retrieve all attributes for a tour


# **get_campsite_attributes**
> ::models::InlineResponse2009 get_campsite_attributes(ctx, campsite_id, optional)
retrieve all attributes for a campsite

This endpoint retrieves all attributes for a specific campsite.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **campsite_id** | **String**| id of the campsite | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **campsite_id** | **String**| id of the campsite | 
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_permit_entrance_attributes**
> ::models::InlineResponse2009 get_permit_entrance_attributes(ctx, permit_entrance_id, optional)
retrieve all attributes for a permit entrance

This endpoint retrieves all attributes for a specific permit entrance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **permit_entrance_id** | **String**| id of the permit entrance | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **permit_entrance_id** | **String**| id of the permit entrance | 
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tour_attributes**
> ::models::InlineResponse2009 get_tour_attributes(ctx, tour_id, optional)
retrieve all attributes for a tour

This endpoint retrieves all attributes for a specific tour.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tour_id** | **String**| id of the tour | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tour_id** | **String**| id of the tour | 
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

