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

pub struct ToursApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ToursApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ToursApiClient {
        ToursApiClient {
            configuration: configuration,
        }
    }
}

pub trait ToursApi {
    fn get_tours(&self, ) -> Result<::models::InlineResponse2007, Error>;
}


impl ToursApi for ToursApiClient {
    fn get_tours(&self, ) -> Result<::models::InlineResponse2007, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/tours?{}", configuration.base_path, query_string);

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
