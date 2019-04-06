# \FacilitiesApi

All URIs are relative to *http://RIDB_HOST/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_facilities**](FacilitiesApi.md#get_facilities) | **Get** /facilities | retrieve all facilities
[**get_organization_facilities**](FacilitiesApi.md#get_organization_facilities) | **Get** /organizations/{orgId}/facilities | retrieve all facilities for an organization
[**get_rec_area_facilities**](FacilitiesApi.md#get_rec_area_facilities) | **Get** /recareas/{recAreaId}/facilities | retrieve all facilities for a recreation area


# **get_facilities**
> ::models::InlineResponse2002 get_facilities(ctx, )
retrieve all facilities

This endpoint retrieves all facilities

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_organization_facilities**
> get_organization_facilities(ctx, )
retrieve all facilities for an organization

This endpoint retrieves all facilities belonging to a specific organization.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_facilities**
> get_rec_area_facilities(ctx, )
retrieve all facilities for a recreation area

This endpoint retrieves all facilities belonging to a specific recreation area.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

