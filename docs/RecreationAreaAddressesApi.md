# \RecreationAreaAddressesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_rec_area_address**](RecreationAreaAddressesApi.md#get_rec_area_address) | **get** /recareaaddresses/{recAreaAddressId} | retrieve a specific recreation area address by id
[**get_rec_area_addresses**](RecreationAreaAddressesApi.md#get_rec_area_addresses) | **get** /recareaaddresses | retrieve all recreation area addresses
[**get_rec_area_rec_area_address**](RecreationAreaAddressesApi.md#get_rec_area_rec_area_address) | **get** /recareas/{recAreaId}/recareaaddresses/{recAreaAddressId} | retrieve a specific recreation area address by id for a recreation area
[**get_rec_area_rec_area_addresses**](RecreationAreaAddressesApi.md#get_rec_area_rec_area_addresses) | **get** /recareas/{recAreaId}/recareaaddresses | retrieve all recreation area addresses for a recreation area


# **get_rec_area_address**
> ::models::RecreationAreaAddress get_rec_area_address(ctx, rec_area_address_id)
retrieve a specific recreation area address by id

This endpoint retrieves a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_address_id** | **String**| id of the recreation area address | 

### Return type

[**::models::RecreationAreaAddress**](Recreation Area Address.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_addresses**
> ::models::InlineResponse2003 get_rec_area_addresses(ctx, optional)
retrieve all recreation area addresses

This endpoint retrieves all recreation area addresses.

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

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_rec_area_address**
> ::models::RecreationAreaAddress get_rec_area_rec_area_address(ctx, rec_area_id, rec_area_address_id)
retrieve a specific recreation area address by id for a recreation area

This endpoint retrieves a specific recreation area for a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 
  **rec_area_address_id** | **String**| id of the recreation area address | 

### Return type

[**::models::RecreationAreaAddress**](Recreation Area Address.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_rec_area_addresses**
> ::models::InlineResponse2003 get_rec_area_rec_area_addresses(ctx, rec_area_id, optional)
retrieve all recreation area addresses for a recreation area

This endpoint retrieves all recreation area addresses for a specific recreation area.

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

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

