# \DefaultApi

All URIs are relative to *https://ridb.recreation.gov/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activity**](DefaultApi.md#get_activity) | **Get** /activities/{activityId} | retrieve a specific activity by id
[**get_all_facility_media**](DefaultApi.md#get_all_facility_media) | **Get** /facilities/{facilityId}/media | retrieve all media for a facility
[**get_all_rec_area_media**](DefaultApi.md#get_all_rec_area_media) | **Get** /recareas/{recAreaId}/media | retrieve all media for a recreation area
[**get_campsite**](DefaultApi.md#get_campsite) | **Get** /campsites/{campsiteId} | retrieve a specific campsite by id
[**get_event**](DefaultApi.md#get_event) | **Get** /events/{eventId} | retrieve a specific event by id
[**get_facility**](DefaultApi.md#get_facility) | **Get** /facilities/{facilityId} | retrieve a specific facility by id
[**get_facility_activities**](DefaultApi.md#get_facility_activities) | **Get** /facilities/{facilityId}/activities | retrieve all activities for a facility
[**get_facility_activity**](DefaultApi.md#get_facility_activity) | **Get** /facilities/{facilityId}/activities/{activityId} | retrieve a specific activity by id for a facility
[**get_facility_address**](DefaultApi.md#get_facility_address) | **Get** /facilityaddresses/{facilityAddressId} | retrieve a specific facility address by id
[**get_facility_campsite**](DefaultApi.md#get_facility_campsite) | **Get** /facilities/{facilityId}/campsites/{campsiteId} | retrieve a specific campsite by id for a facility
[**get_facility_campsites**](DefaultApi.md#get_facility_campsites) | **Get** /facilities/{facilityId}/campsites | retrieve all campsites for a facility
[**get_facility_event**](DefaultApi.md#get_facility_event) | **Get** /facilities/{facilityId}/events/{eventId} | retrieve a specific event by id for a facility
[**get_facility_events**](DefaultApi.md#get_facility_events) | **Get** /facilities/{facilityId}/events | retrieve all events for a facility
[**get_facility_facility_address**](DefaultApi.md#get_facility_facility_address) | **Get** /facilities/{facilityId}/facilityaddresses/{facilityAddressId} | retrieve a specific facility address by id for a facility
[**get_facility_facility_addresses**](DefaultApi.md#get_facility_facility_addresses) | **Get** /facilities/{facilityId}/facilityaddresses | retrieve all facility addresses for a facility
[**get_facility_link**](DefaultApi.md#get_facility_link) | **Get** /facilities/{facilityId}/links/{linkId} | retrieve a specific link by id for a facility
[**get_facility_links**](DefaultApi.md#get_facility_links) | **Get** /facilities/{facilityId}/links | retrieve all links for a facility
[**get_facility_media**](DefaultApi.md#get_facility_media) | **Get** /facilities/{facilityId}/media/{mediaId} | retrieve a specific media by id for a facility
[**get_facility_permit_entrance**](DefaultApi.md#get_facility_permit_entrance) | **Get** /facilities/{facilityId}/permitentrances/{permitEntranceId} | retrieve a specific permit entrance by id for a facility
[**get_facility_permit_entrances**](DefaultApi.md#get_facility_permit_entrances) | **Get** /facilities/{facilityId}/permitentrances | retrieve all permit entrances for a facility
[**get_facility_tour**](DefaultApi.md#get_facility_tour) | **Get** /facilities/{facilityId}/tours/{tourId} | retrieve a specific tour by id for a facility
[**get_facility_tours**](DefaultApi.md#get_facility_tours) | **Get** /facilities/{facilityId}/tours | retrieve all tours for a facility
[**get_link**](DefaultApi.md#get_link) | **Get** /links/{linkId} | retrieve a specific link by id
[**get_media**](DefaultApi.md#get_media) | **Get** /media/{mediaId} | retrieve a specific media by id
[**get_organization**](DefaultApi.md#get_organization) | **Get** /organizations/{orgId} | retrieve a specific organization by id
[**get_organization_facility**](DefaultApi.md#get_organization_facility) | **Get** /organizations/{orgId}/facilities/{facilityId} | retrieve a specific facility by id for an organization
[**get_organization_rec_area**](DefaultApi.md#get_organization_rec_area) | **Get** /organizations/{orgId}/recareas/{recAreaId} | retrieve a specific recreation area by id for an organization
[**get_organization_rec_areas**](DefaultApi.md#get_organization_rec_areas) | **Get** /organizations/{orgId}/recareas | retrieve all recreation areas for an organization
[**get_permit_entrance**](DefaultApi.md#get_permit_entrance) | **Get** /permitentrances/{permitentranceId} | retrieve a specific permit entrance by id
[**get_permit_entrance_attributes**](DefaultApi.md#get_permit_entrance_attributes) | **Get** /permitentrances/{permitEntranceId}/attributes | retrieve all attributes for a permit entrance
[**get_rec_area**](DefaultApi.md#get_rec_area) | **Get** /recareas/{recAreaId} | retrieve a specific recreation area by id
[**get_rec_area_activities**](DefaultApi.md#get_rec_area_activities) | **Get** /recareas/{recAreaId}/activities | retrieve all activities for a recreation area
[**get_rec_area_activity**](DefaultApi.md#get_rec_area_activity) | **Get** /recareas/{recAreaId}/activities/{activityId} | retrieve a specific activity by id for a recreation area
[**get_rec_area_address**](DefaultApi.md#get_rec_area_address) | **Get** /recareaaddresses/{recAreaAddressId} | retrieve a specific recreation area address by id
[**get_rec_area_event**](DefaultApi.md#get_rec_area_event) | **Get** /recareas/{recAreaId}/events/{eventId} | retrieve a specific event by id for a recreation area
[**get_rec_area_events**](DefaultApi.md#get_rec_area_events) | **Get** /recareas/{recAreaId}/events | retrieve all events for a recreation area
[**get_rec_area_facility**](DefaultApi.md#get_rec_area_facility) | **Get** /recareas/{recAreaId}/facilities/{facilityId} | retrieve a specific facility by id for a recreation area
[**get_rec_area_link**](DefaultApi.md#get_rec_area_link) | **Get** /recareas/{recAreaId}/links/{linkId} | retrieve a specific link by id for a recreation area
[**get_rec_area_links**](DefaultApi.md#get_rec_area_links) | **Get** /recareas/{recAreaId}/links | retrieve all links for a recreation area
[**get_rec_area_media**](DefaultApi.md#get_rec_area_media) | **Get** /recareas/{recAreaId}/media/{mediaId} | retrieve a specific media by id for a recreation area
[**get_rec_area_rec_area_address**](DefaultApi.md#get_rec_area_rec_area_address) | **Get** /recareas/{recAreaId}/recareaaddresses/{recAreaAddressId} | retrieve a specific recreation area address by id for a recreation area
[**get_rec_area_rec_area_addresses**](DefaultApi.md#get_rec_area_rec_area_addresses) | **Get** /recareas/{recAreaId}/recareaaddresses | retrieve all recreation area addresses for a recreation area
[**get_tour**](DefaultApi.md#get_tour) | **Get** /tours/{tourId} | retrieve a specific tour by id


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

# **get_all_facility_media**
> get_all_facility_media(ctx, )
retrieve all media for a facility

This endpoint retrieves all media for a specific facility.

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

# **get_all_rec_area_media**
> get_all_rec_area_media(ctx, )
retrieve all media for a recreation area

This endpoint retrieves all media for a specific recreation area.

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

# **get_campsite**
> ::models::Campsite get_campsite(ctx, campsite_id)
retrieve a specific campsite by id

This endpoint retrieves a specific campsite.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **campsite_id** | **String**| id of the campsite | 

### Return type

[**::models::Campsite**](Campsite.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_event**
> ::models::Event get_event(ctx, event_id)
retrieve a specific event by id

This endpoint retrieves a specific event.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **event_id** | **String**| id of the event | 

### Return type

[**::models::Event**](Event.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility**
> ::models::Facility get_facility(ctx, facility_id)
retrieve a specific facility by id

This endpoint retrieves a specific facility.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **facility_id** | **String**| id of the facility | 

### Return type

[**::models::Facility**](Facility.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_facility_activities**
> get_facility_activities(ctx, )
retrieve all activities for a facility

This endpoint retrieves all activities for a specific facility.

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

# **get_facility_activity**
> get_facility_activity(ctx, )
retrieve a specific activity by id for a facility

This endpoint retrieves a specific activity for a specific facility.

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

# **get_facility_campsite**
> get_facility_campsite(ctx, )
retrieve a specific campsite by id for a facility

This endpoint retrieves a specific campsite for a specific facility.

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

# **get_facility_campsites**
> get_facility_campsites(ctx, )
retrieve all campsites for a facility

This endpoint retrieves all campsites for a specific facility.

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

# **get_facility_event**
> get_facility_event(ctx, )
retrieve a specific event by id for a facility

This endpoint retrieves a specific event for a specific facility.

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

# **get_facility_events**
> get_facility_events(ctx, )
retrieve all events for a facility

This endpoint retrieves all events for a specific facility.

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

# **get_facility_facility_address**
> get_facility_facility_address(ctx, )
retrieve a specific facility address by id for a facility

This endpoint retrieves a specific facility for a specific facility.

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

# **get_facility_facility_addresses**
> get_facility_facility_addresses(ctx, )
retrieve all facility addresses for a facility

This endpoint retrieves all facility addresses for a specific facility.

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

# **get_facility_link**
> get_facility_link(ctx, )
retrieve a specific link by id for a facility

This endpoint retrieves a specific link for a specific facility.

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

# **get_facility_links**
> get_facility_links(ctx, )
retrieve all links for a facility

This endpoint retrieves all links for a specific facility.

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

# **get_facility_media**
> get_facility_media(ctx, )
retrieve a specific media by id for a facility

This endpoint retrieves a specific media for a specific facility.

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

# **get_facility_permit_entrance**
> get_facility_permit_entrance(ctx, )
retrieve a specific permit entrance by id for a facility

This endpoint retrieves a specific permit entrance for a specific facility.

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

# **get_facility_permit_entrances**
> get_facility_permit_entrances(ctx, )
retrieve all permit entrances for a facility

This endpoint retrieves all permit entrances for a specific facility.

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

# **get_facility_tour**
> get_facility_tour(ctx, )
retrieve a specific tour by id for a facility

This endpoint retrieves a specific tour for a specific facility.

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

# **get_facility_tours**
> get_facility_tours(ctx, )
retrieve all tours for a facility

This endpoint retrieves all tours for a specific facility.

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

# **get_link**
> ::models::Link get_link(ctx, link_id)
retrieve a specific link by id

This endpoint retrieves a specific link.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **link_id** | **String**| id of the link | 

### Return type

[**::models::Link**](Link.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_media**
> ::models::Media get_media(ctx, media_id)
retrieve a specific media by id

This endpoint retrieves a specific media.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **media_id** | **String**| id of the media | 

### Return type

[**::models::Media**](Media.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_organization**
> Vec<::models::Organization> get_organization(ctx, org_id)
retrieve a specific organization by id

This endpoint retrieves a specific organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **org_id** | **String**| id of the organization | 

### Return type

[**Vec<::models::Organization>**](Organization.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_organization_facility**
> get_organization_facility(ctx, )
retrieve a specific facility by id for an organization

This endpoint retrieves a specific facility belonging to a specific organization.

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

# **get_organization_rec_area**
> get_organization_rec_area(ctx, )
retrieve a specific recreation area by id for an organization

This endpoint retrieves a specific recreation area belonging to a specific organization.

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

# **get_organization_rec_areas**
> get_organization_rec_areas(ctx, )
retrieve all recreation areas for an organization

This endpoint retrieves all recreation areas belonging to a specific organization.

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

# **get_permit_entrance**
> ::models::PermitEntrance get_permit_entrance(ctx, permit_entrance_id)
retrieve a specific permit entrance by id

This endpoint retrieves a specific permit entrance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **permit_entrance_id** | **String**| id of the permit entrance | 

### Return type

[**::models::PermitEntrance**](Permit Entrance.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_permit_entrance_attributes**
> get_permit_entrance_attributes(ctx, )
retrieve all attributes for a permit entrance

This endpoint retrieves all attributes for a specific permit entrance.

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

# **get_rec_area**
> ::models::RecreationArea get_rec_area(ctx, rec_area_id)
retrieve a specific recreation area by id

This endpoint retrieves a specific recreation area.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rec_area_id** | **String**| id of the recreation area | 

### Return type

[**::models::RecreationArea**](Recreation Area.md)

### Authorization

[Apikey](../README.md#Apikey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rec_area_activities**
> get_rec_area_activities(ctx, )
retrieve all activities for a recreation area

This endpoint retrieves all activities for a specific recreation area.

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

# **get_rec_area_activity**
> get_rec_area_activity(ctx, )
retrieve a specific activity by id for a recreation area

This endpoint retrieves a specific activity for a specific recreation area.

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

# **get_rec_area_event**
> get_rec_area_event(ctx, )
retrieve a specific event by id for a recreation area

This endpoint retrieves a specific event for a specific recreation area.

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

# **get_rec_area_events**
> get_rec_area_events(ctx, )
retrieve all events for a recreation area

This endpoint retrieves all events for a specific recreation area.

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

# **get_rec_area_facility**
> get_rec_area_facility(ctx, )
retrieve a specific facility by id for a recreation area

This endpoint retrieves a specific facility belonging to a specific recreation area.

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

# **get_rec_area_link**
> get_rec_area_link(ctx, )
retrieve a specific link by id for a recreation area

This endpoint retrieves a specific link for a specific recreation area.

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

# **get_rec_area_links**
> get_rec_area_links(ctx, )
retrieve all links for a recreation area

This endpoint retrieves all links for a specific recreation area.

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

# **get_rec_area_media**
> get_rec_area_media(ctx, )
retrieve a specific media by id for a recreation area

This endpoint retrieves a specific media for a specific recreation area.

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

# **get_rec_area_rec_area_address**
> get_rec_area_rec_area_address(ctx, )
retrieve a specific recreation area address by id for a recreation area

This endpoint retrieves a specific recreation area for a specific recreation area.

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

# **get_rec_area_rec_area_addresses**
> get_rec_area_rec_area_addresses(ctx, )
retrieve all recreation area addresses for a recreation area

This endpoint retrieves all recreation area addresses for a specific recreation area.

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

