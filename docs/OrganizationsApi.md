# \OrganizationsApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organizations**](OrganizationsApi.md#get_organizations) | **Get** /organizations | retrieve all organizations


# **get_organizations**
> ::models::InlineResponse200 get_organizations(ctx, optional)
retrieve all organizations

This endpoint retrieves all organizations.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| query filter criteria, Searches on organization name and organization abbreviated name | 
 **limit** | **i32**| number of records to return (max 50) | [default to 50]
 **offset** | **i32**| start record of overall result set | [default to 0]

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

