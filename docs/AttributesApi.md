# \AttributesApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_campsite_attributes**](AttributesApi.md#get_campsite_attributes) | **get** /campsites/{campsiteId}/attributes | retrieve all attributes for a campsite
[**get_tour_attributes**](AttributesApi.md#get_tour_attributes) | **get** /tours/{tourId}/attributes | retrieve all attributes for a tour


# **get_campsite_attributes**
> ::models::InlineResponse2009 get_campsite_attributes(ctx, )
retrieve all attributes for a campsite

This endpoint retrieves all attributes for a specific campsite.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tour_attributes**
> get_tour_attributes(ctx, )
retrieve all attributes for a tour

This endpoint retrieves all attributes for a specific tour.

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

