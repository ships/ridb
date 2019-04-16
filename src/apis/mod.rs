use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod activities_api;
pub use self::activities_api::{ ActivitiesApi, ActivitiesApiClient };
mod attributes_api;
pub use self::attributes_api::{ AttributesApi, AttributesApiClient };
mod campsites_api;
pub use self::campsites_api::{ CampsitesApi, CampsitesApiClient };
mod default_api;
pub use self::default_api::{ DefaultApi, DefaultApiClient };
mod events_api;
pub use self::events_api::{ EventsApi, EventsApiClient };
mod facilities_api;
pub use self::facilities_api::{ FacilitiesApi, FacilitiesApiClient };
mod facility_addresses_api;
pub use self::facility_addresses_api::{ FacilityAddressesApi, FacilityAddressesApiClient };
mod links_api;
pub use self::links_api::{ LinksApi, LinksApiClient };
mod media_api;
pub use self::media_api::{ MediaApi, MediaApiClient };
mod organizations_api;
pub use self::organizations_api::{ OrganizationsApi, OrganizationsApiClient };
mod permit_entrances_api;
pub use self::permit_entrances_api::{ PermitEntrancesApi, PermitEntrancesApiClient };
mod recreation_area_addresses_api;
pub use self::recreation_area_addresses_api::{ RecreationAreaAddressesApi, RecreationAreaAddressesApiClient };
mod recreation_areas_api;
pub use self::recreation_areas_api::{ RecreationAreasApi, RecreationAreasApiClient };
mod tours_api;
pub use self::tours_api::{ ToursApi, ToursApiClient };
mod zones_api;
pub use self::zones_api::{ ZonesApi, ZonesApiClient };

pub mod configuration;
pub mod client;
