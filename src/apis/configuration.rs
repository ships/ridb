/* 
 * RIDB API
 *
 * The Recreation Information Database (RIDB) provides data resources to citizens, offering a single point of access to information about recreational opportunities nationwide. The RIDB represents an authoritative source of information and services for millions of visitors to federal lands, historic sites, museums, and other attractions/resources. This initiative integrates multiple Federal channels and sources about recreation opportunities into a one-stop, searchable database of recreational areas nationwide.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use hyper;
use std::collections::HashMap;

pub struct Configuration<C: hyper::client::Connect> {
  pub base_path: String,
  pub user_agent: Option<String>,
  pub client: hyper::client::Client<C>,
  pub basic_auth: Option<BasicAuth>,
  pub oauth_access_token: Option<String>,
  pub api_key: Option<ApiKey>,
  // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
  pub prefix: Option<String>,
  pub key: String,
}

impl<C: hyper::client::Connect> Configuration<C> {
  pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
    Configuration {
      base_path: "https://ridb.recreation.gov/api/v1".to_owned(),
      user_agent: Some("OpenAPI-Generator/1.0.0/rust".to_owned()),
      client: client,
      basic_auth: None,
      oauth_access_token: None,
      api_key: None,
    }
  }
}
