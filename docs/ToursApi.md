# \ToursApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_facility_tour**](ToursApi.md#get_facility_tour) | **get** /facilities/{facilityId}/tours/{tourId} | retrieve a specific tour by id for a facility
[**get_facility_tours**](ToursApi.md#get_facility_tours) | **get** /facilities/{facilityId}/tours | retrieve all tours for a facility
[**get_tour**](ToursApi.md#get_tour) | **get** /tours/{tourId} | retrieve a specific tour by id
[**get_tours**](ToursApi.md#get_tours) | **get** /tours | retrieve all tours


# **get_facility_tour**
> ::models::Tour get_facility_tour(ctx, facility_id, tour_id)
retrieve a specific tour by id for a facility

This endpoint retrieves a specific tour for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **tour_id** | **String**| id of the tour | 

### Return type

[**::models::Tour**](Tour.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_tours**
> ::models::InlineResponse2007 get_facility_tours(ctx, facility_id, optional)
retrieve all tours for a facility

This endpoint retrieves all tours for a specific facility.

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

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tour**
> ::models::Tour get_tour(ctx, tour_id)
retrieve a specific tour by id

This endpoint retrieves a specific tour.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **tour_id** | **String**| id of the tour | 

### Return type

[**::models::Tour**](Tour.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tours**
> ::models::InlineResponse2007 get_tours(ctx, optional)
retrieve all tours

This endpoint retrieves all tours

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

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

