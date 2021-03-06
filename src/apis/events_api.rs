/* 
 * RIDB API
 *
 * The Recreation Information Database (RIDB) provides data resources to citizens, offering a single point of access to information about recreational opportunities nationwide. The RIDB represents an authoritative source of information and services for millions of visitors to federal lands, historic sites, museums, and other attractions/resources. This initiative integrates multiple Federal channels and sources about recreation opportunities into a one-stop, searchable database of recreational areas nationwide.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct EventsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl EventsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> EventsApiClient {
        EventsApiClient {
            configuration: configuration,
        }
    }
}

pub trait EventsApi {
    fn get_event(&self, event_id: &str) -> Result<::models::Event, Error>;
    fn get_events(&self, limit: i32, offset: i32) -> Result<::models::InlineResponse20011, Error>;
    fn get_facility_event(&self, facility_id: &str, event_id: &str) -> Result<::models::Event, Error>;
    fn get_facility_events(&self, facility_id: &str, limit: i32, offset: i32) -> Result<::models::InlineResponse20011, Error>;
    fn get_rec_area_event(&self, rec_area_id: &str, event_id: &str) -> Result<::models::Event, Error>;
    fn get_rec_area_events(&self, rec_area_id: &str, limit: i32, offset: i32) -> Result<::models::InlineResponse20011, Error>;
}


impl EventsApi for EventsApiClient {
    fn get_event(&self, event_id: &str) -> Result<::models::Event, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/events/{eventId}?{}", configuration.base_path, query_string, eventId=event_id);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("apikey", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_events(&self, limit: i32, offset: i32) -> Result<::models::InlineResponse20011, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("offset", &offset.to_string());

            query.finish()
        };
        let uri_str = format!("{}/events?{}", configuration.base_path, query_string);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("apikey", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_facility_event(&self, facility_id: &str, event_id: &str) -> Result<::models::Event, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/facilities/{facilityId}/events/{eventId}?{}", configuration.base_path, query_string, facilityId=facility_id, eventId=event_id);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("apikey", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_facility_events(&self, facility_id: &str, limit: i32, offset: i32) -> Result<::models::InlineResponse20011, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("offset", &offset.to_string());

            query.finish()
        };
        let uri_str = format!("{}/facilities/{facilityId}/events?{}", configuration.base_path, query_string, facilityId=facility_id);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("apikey", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_rec_area_event(&self, rec_area_id: &str, event_id: &str) -> Result<::models::Event, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/recareas/{recAreaId}/events/{eventId}?{}", configuration.base_path, query_string, recAreaId=rec_area_id, eventId=event_id);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("apikey", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_rec_area_events(&self, rec_area_id: &str, limit: i32, offset: i32) -> Result<::models::InlineResponse20011, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("limit", &limit.to_string());
            query.append_pair("offset", &offset.to_string());

            query.finish()
        };
        let uri_str = format!("{}/recareas/{recAreaId}/events?{}", configuration.base_path, query_string, recAreaId=rec_area_id);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }


        
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("apikey", val);
        };
        


        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
