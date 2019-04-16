use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
  configuration: Rc<Configuration>,
  activities_api: Box<::apis::ActivitiesApi>,
  attributes_api: Box<::apis::AttributesApi>,
  campsites_api: Box<::apis::CampsitesApi>,
  events_api: Box<::apis::EventsApi>,
  facilities_api: Box<::apis::FacilitiesApi>,
  facility_addresses_api: Box<::apis::FacilityAddressesApi>,
  links_api: Box<::apis::LinksApi>,
  media_api: Box<::apis::MediaApi>,
  organizations_api: Box<::apis::OrganizationsApi>,
  permit_entrances_api: Box<::apis::PermitEntrancesApi>,
  recreation_area_addresses_api: Box<::apis::RecreationAreaAddressesApi>,
  recreation_areas_api: Box<::apis::RecreationAreasApi>,
  tours_api: Box<::apis::ToursApi>,
  zones_api: Box<::apis::ZonesApi>,
}

impl APIClient {
  pub fn new(configuration: Configuration) -> APIClient {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      activities_api: Box::new(::apis::ActivitiesApiClient::new(rc.clone())),
      attributes_api: Box::new(::apis::AttributesApiClient::new(rc.clone())),
      campsites_api: Box::new(::apis::CampsitesApiClient::new(rc.clone())),
      events_api: Box::new(::apis::EventsApiClient::new(rc.clone())),
      facilities_api: Box::new(::apis::FacilitiesApiClient::new(rc.clone())),
      facility_addresses_api: Box::new(::apis::FacilityAddressesApiClient::new(rc.clone())),
      links_api: Box::new(::apis::LinksApiClient::new(rc.clone())),
      media_api: Box::new(::apis::MediaApiClient::new(rc.clone())),
      organizations_api: Box::new(::apis::OrganizationsApiClient::new(rc.clone())),
      permit_entrances_api: Box::new(::apis::PermitEntrancesApiClient::new(rc.clone())),
      recreation_area_addresses_api: Box::new(::apis::RecreationAreaAddressesApiClient::new(rc.clone())),
      recreation_areas_api: Box::new(::apis::RecreationAreasApiClient::new(rc.clone())),
      tours_api: Box::new(::apis::ToursApiClient::new(rc.clone())),
      zones_api: Box::new(::apis::ZonesApiClient::new(rc.clone())),
    }
  }

  pub fn activities_api(&self) -> &::apis::ActivitiesApi{
    self.activities_api.as_ref()
  }

  pub fn attributes_api(&self) -> &::apis::AttributesApi{
    self.attributes_api.as_ref()
  }

  pub fn campsites_api(&self) -> &::apis::CampsitesApi{
    self.campsites_api.as_ref()
  }

  pub fn events_api(&self) -> &::apis::EventsApi{
    self.events_api.as_ref()
  }

  pub fn facilities_api(&self) -> &::apis::FacilitiesApi{
    self.facilities_api.as_ref()
  }

  pub fn facility_addresses_api(&self) -> &::apis::FacilityAddressesApi{
    self.facility_addresses_api.as_ref()
  }

  pub fn links_api(&self) -> &::apis::LinksApi{
    self.links_api.as_ref()
  }

  pub fn media_api(&self) -> &::apis::MediaApi{
    self.media_api.as_ref()
  }

  pub fn organizations_api(&self) -> &::apis::OrganizationsApi{
    self.organizations_api.as_ref()
  }

  pub fn permit_entrances_api(&self) -> &::apis::PermitEntrancesApi{
    self.permit_entrances_api.as_ref()
  }

  pub fn recreation_area_addresses_api(&self) -> &::apis::RecreationAreaAddressesApi{
    self.recreation_area_addresses_api.as_ref()
  }

  pub fn recreation_areas_api(&self) -> &::apis::RecreationAreasApi{
    self.recreation_areas_api.as_ref()
  }

  pub fn tours_api(&self) -> &::apis::ToursApi{
    self.tours_api.as_ref()
  }

  pub fn zones_api(&self) -> &::apis::ZonesApi{
    self.zones_api.as_ref()
  }


}
