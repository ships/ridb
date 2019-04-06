# \ZonesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_permit_entrance_zone**](ZonesApi.md#get_permit_entrance_zone) | **Get** /permitentrances/{permitEntranceId}/zones/{zoneId} | retrieve a zone for a permit entrance
[**get_permit_entrance_zones**](ZonesApi.md#get_permit_entrance_zones) | **Get** /permitentrances/{permitEntranceId}/zones | retrieve all zones for a permit entrance


# **get_permit_entrance_zone**
> ::models::Zone get_permit_entrance_zone(ctx, zone_id)
retrieve a zone for a permit entrance

This endpoint retrieves a specific zone for a specific permit entrance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **zone_id** | **String**| id of the zone | 

### Return type

[**::models::Zone**](Zone.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_permit_entrance_zones**
> ::models::InlineResponse20010 get_permit_entrance_zones(ctx, )
retrieve all zones for a permit entrance

This endpoint retrieves all zones for a specific permit entrance.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

