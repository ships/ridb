# \ActivitiesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activities**](ActivitiesApi.md#get_activities) | **get** /activities | retrieve all activities
[**get_activity**](ActivitiesApi.md#get_activity) | **get** /activities/{activityId} | retrieve a specific activity by id
[**get_facility_activities**](ActivitiesApi.md#get_facility_activities) | **get** /facilities/{facilityId}/activities | retrieve all activities for a facility
[**get_facility_activity**](ActivitiesApi.md#get_facility_activity) | **get** /facilities/{facilityId}/activities/{activityId} | retrieve a specific activity by id for a facility
[**get_rec_area_activities**](ActivitiesApi.md#get_rec_area_activities) | **get** /recareas/{recAreaId}/activities | retrieve all activities for a recreation area
[**get_rec_area_activity**](ActivitiesApi.md#get_rec_area_activity) | **get** /recareas/{recAreaId}/activities/{activityId} | retrieve a specific activity by id for a recreation area


# **get_activities**
> ::models::InlineResponse2008 get_activities(ctx, optional)
retrieve all activities

This endpoint retrieves all activities.

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

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_activity**
> ::models::Activity get_activity(ctx, activity_id)
retrieve a specific activity by id

This endpoint retrieves a specific activity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **activity_id** | **String**| id of the activity | 

### Return type

[**::models::Activity**](Activity.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_activities**
> ::models::InlineResponse2008 get_facility_activities(ctx, facility_id, optional)
retrieve all activities for a facility

This endpoint retrieves all activities for a specific facility.

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

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_activity**
> ::models::Activity get_facility_activity(ctx, facility_id, activity_id)
retrieve a specific activity by id for a facility

This endpoint retrieves a specific activity for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **activity_id** | **String**| id of the activity | 

### Return type

[**::models::Activity**](Activity.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_activities**
> ::models::InlineResponse2008 get_rec_area_activities(ctx, rec_area_id, optional)
retrieve all activities for a recreation area

This endpoint retrieves all activities for a specific recreation area.

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

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_activity**
> ::models::Activity get_rec_area_activity(ctx, rec_area_id, activity_id)
retrieve a specific activity by id for a recreation area

This endpoint retrieves a specific activity for a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 
  **activity_id** | **String**| id of the activity | 

### Return type

[**::models::Activity**](Activity.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

