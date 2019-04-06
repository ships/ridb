use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod request;

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
