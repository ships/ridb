# Facility

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**facility_id** | **String** |  | 
**legacy_facility_id** | **String** |  | 
**org_facility_id** | **String** |  | 
**parent_org_id** | **String** |  | [optional] 
**parent_rec_area_id** | **String** |  | [optional] 
**facility_name** | **String** |  | 
**facility_description** | **String** |  | 
**facility_type_description** | **String** |  | 
**facility_use_fee_description** | **String** |  | 
**facility_directions** | **String** |  | 
**facility_phone** | **String** |  | 
**facility_email** | **String** |  | 
**facility_reservation_url** | **String** |  | 
**facility_map_url** | **String** |  | 
**facility_ada_access** | **String** |  | 
**GEOJSON** | [***::models::FacilityGeojson**](Facility_GEOJSON.md) |  | 
**facility_longitude** | **f64** |  | 
**facility_latitude** | **f64** |  | 
**stay_limit** | **String** |  | 
**keywords** | **String** |  | 
**reservable** | **bool** |  | 
**enabled** | **bool** |  | 
**CAMPSITE** | [**Vec<::models::FacilityCampsite>**](Facility Campsite.md) |  | [optional] 
**PERMITENTRANCE** | [**Vec<::models::FacilityPermitEntrance>**](Facility Permit Entrance.md) |  | [optional] 
**TOUR** | [**Vec<::models::FacilityTour>**](Facility Tour.md) |  | [optional] 
**ORGANIZATION** | [**Vec<::models::Organization>**](Organization.md) |  | [optional] 
**RECAREA** | [**Vec<::models::FacilityRecArea>**](Facility RecArea.md) |  | [optional] 
**FACILITYADDRESS** | [**Vec<::models::FacilityAddress>**](Facility Address.md) |  | [optional] 
**ACTIVITY** | [**Vec<::models::FacilityActivity>**](Facility Activity.md) |  | [optional] 
**EVENT** | [**Vec<::models::Event>**](Event.md) |  | [optional] 
**LINK** | [**Vec<::models::Link>**](Link.md) |  | [optional] 
**MEDIA** | [**Vec<::models::Media>**](Media.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


