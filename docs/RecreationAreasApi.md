# \RecreationAreasApi

All URIs are relative to *http://RIDB_HOST/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_rec_areas**](RecreationAreasApi.md#get_rec_areas) | **Get** /recareas | retrieve all recreation areas


# **get_rec_areas**
> ::models::InlineResponse2001 get_rec_areas(ctx, optional)
retrieve all recreation areas

This endpoint retrieves all recreation areas.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **full** | **String**| return the full record details or compact (abbreviated) details | 
 **state** | [**Vec<String>**](String.md)| comma delimited list of 2 character state codes | 
 **activity** | [**Vec<String>**](String.md)| comma delimited list of activity IDs or keyword | 
 **latitude** | **f64**| Latitude of the point in decimal degrees | 
 **longitude** | **f64**| Longitude of the point in decimal degrees | 
 **radius** | **f64**| Distance (in miles) by which to include search results | [default to 25]
 **lastupdated** | **String**| return all records modified since this date **Date Format: (mm-dd-yyyy)** | 
 **sort** | **String**| sort the results by \"ID\", \"Date\" or \"Name\" | 

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

