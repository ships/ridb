# \FacilityAddressesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_facility_address**](FacilityAddressesApi.md#get_facility_address) | **get** /facilityaddresses/{facilityAddressId} | retrieve a specific facility address by id
[**get_facility_addresses**](FacilityAddressesApi.md#get_facility_addresses) | **get** /facilityaddresses | retrieve all facility addresses
[**get_facility_facility_address**](FacilityAddressesApi.md#get_facility_facility_address) | **get** /facilities/{facilityId}/facilityaddresses/{facilityAddressId} | retrieve a specific facility address by id for a facility
[**get_facility_facility_addresses**](FacilityAddressesApi.md#get_facility_facility_addresses) | **get** /facilities/{facilityId}/facilityaddresses | retrieve all facility addresses for a facility


# **get_facility_address**
> ::models::FacilityAddress get_facility_address(ctx, facility_address_id)
retrieve a specific facility address by id

This endpoint retrieves a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_address_id** | **String**| id of the facility address | 

### Return type

[**::models::FacilityAddress**](Facility Address.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_addresses**
> ::models::InlineResponse2004 get_facility_addresses(ctx, optional)
retrieve all facility addresses

This endpoint retrieves all facility addresses.

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

[**::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_facility_address**
> ::models::FacilityAddress get_facility_facility_address(ctx, facility_id, facility_address_id)
retrieve a specific facility address by id for a facility

This endpoint retrieves a specific facility for a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 
  **facility_address_id** | **String**| id of the facility address | 

### Return type

[**::models::FacilityAddress**](Facility Address.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_facility_addresses**
> ::models::InlineResponse2004 get_facility_facility_addresses(ctx, facility_id, optional)
retrieve all facility addresses for a facility

This endpoint retrieves all facility addresses for a specific facility.

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

[**::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

