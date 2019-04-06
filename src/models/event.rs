/* 
 * RIDB API
 *
 * The Recreation Information Database (RIDB) provides data resources to citizens, offering a single point of access to information about recreational opportunities nationwide. The RIDB represents an authoritative source of information and services for millions of visitors to federal lands, historic sites, museums, and other attractions/resources. This initiative integrates multiple Federal channels and sources about recreation opportunities into a one-stop, searchable database of recreational areas nationwide.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
  #[serde(rename = "EventID")]
  event_id: String,
  #[serde(rename = "EventName")]
  event_name: String,
  #[serde(rename = "ResourceLink")]
  resource_link: String
}

impl Event {
  pub fn new(event_id: String, event_name: String, resource_link: String) -> Event {
    Event {
      event_id: event_id,
      event_name: event_name,
      resource_link: resource_link
    }
  }

  pub fn set_event_id(&mut self, event_id: String) {
    self.event_id = event_id;
  }

  pub fn with_event_id(mut self, event_id: String) -> Event {
    self.event_id = event_id;
    self
  }

  pub fn event_id(&self) -> &String {
    &self.event_id
  }


  pub fn set_event_name(&mut self, event_name: String) {
    self.event_name = event_name;
  }

  pub fn with_event_name(mut self, event_name: String) -> Event {
    self.event_name = event_name;
    self
  }

  pub fn event_name(&self) -> &String {
    &self.event_name
  }


  pub fn set_resource_link(&mut self, resource_link: String) {
    self.resource_link = resource_link;
  }

  pub fn with_resource_link(mut self, resource_link: String) -> Event {
    self.resource_link = resource_link;
    self
  }

  pub fn resource_link(&self) -> &String {
    &self.resource_link
  }


}



