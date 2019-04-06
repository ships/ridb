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

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct OrganizationsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> OrganizationsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> OrganizationsApiClient<C> {
        OrganizationsApiClient {
            configuration: configuration,
        }
    }
}

pub trait OrganizationsApi {
    fn get_organizations(&self, query: &str, limit: i32, offset: i32) -> Box<Future<Item = ::models::InlineResponse200, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>OrganizationsApi for OrganizationsApiClient<C> {
    fn get_organizations(&self, query: &str, limit: i32, offset: i32) -> Box<Future<Item = ::models::InlineResponse200, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/organizations".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "apikey".to_owned(),
            }))
            .with_query_param("query".to_string(), query.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("offset".to_string(), offset.to_string())
            .execute(self.configuration.borrow())
    }

}
