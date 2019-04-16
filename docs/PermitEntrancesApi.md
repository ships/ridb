# \PermitEntrancesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_facility_permit_entrance**](PermitEntrancesApi.md#get_facility_permit_entrance) | **get** /facilities/{facilityId}/permitentrances/{permitEntranceId} | retrieve a specific permit entrance by id for a facility
[**get_facility_permit_entrances**](PermitEntrancesApi.md#get_facility_permit_entrances) | **get** /facilities/{facilityId}/permitentrances | retrieve all permit entrances for a facility
[**get_permit_entrance**](PermitEntrancesApi.md#get_permit_entrance) | **get** /permitentrances/{permitEntranceId} | retrieve a specific permit entrance by id
[**get_permit_entrances**](PermitEntrancesApi.md#get_permit_entrances) | **get** /permitentrances | retrieve all permit entrances


# **get_facility_permit_entrance**
> Vec<::models::PermitEntrance> get_facility_permit_entrance(ctx, facility_id, permit_entrance_id)
retrieve a specific permit entrance by id for a facility

This endpoint retrieves a specific permit entrance for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **permit_entrance_id** | **String**| id of the permit entrance | 

### Return type

[**Vec<::models::PermitEntrance>**](Permit Entrance.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_permit_entrances**
> ::models::InlineResponse2006 get_facility_permit_entrances(ctx, facility_id, optional)
retrieve all permit entrances for a facility

This endpoint retrieves all permit entrances for a specific facility.

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

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_permit_entrance**
> Vec<::models::PermitEntrance> get_permit_entrance(ctx, permit_entrance_id)
retrieve a specific permit entrance by id

This endpoint retrieves a specific permit entrance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **permit_entrance_id** | **String**| id of the permit entrance | 

### Return type

[**Vec<::models::PermitEntrance>**](Permit Entrance.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_permit_entrances**
> ::models::InlineResponse2006 get_permit_entrances(ctx, optional)
retrieve all permit entrances

This endpoint retrieves all permit entrances

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

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

