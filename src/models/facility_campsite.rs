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
pub struct FacilityCampsite {
  #[serde(rename = "CampsiteID")]
  campsite_id: String,
  #[serde(rename = "CampsiteName")]
  campsite_name: String,
  #[serde(rename = "ResourceLink")]
  resource_link: String
}

impl FacilityCampsite {
  pub fn new(campsite_id: String, campsite_name: String, resource_link: String) -> FacilityCampsite {
    FacilityCampsite {
      campsite_id: campsite_id,
      campsite_name: campsite_name,
      resource_link: resource_link
    }
  }

  pub fn set_campsite_id(&mut self, campsite_id: String) {
    self.campsite_id = campsite_id;
  }

  pub fn with_campsite_id(mut self, campsite_id: String) -> FacilityCampsite {
    self.campsite_id = campsite_id;
    self
  }

  pub fn campsite_id(&self) -> &String {
    &self.campsite_id
  }


  pub fn set_campsite_name(&mut self, campsite_name: String) {
    self.campsite_name = campsite_name;
  }

  pub fn with_campsite_name(mut self, campsite_name: String) -> FacilityCampsite {
    self.campsite_name = campsite_name;
    self
  }

  pub fn campsite_name(&self) -> &String {
    &self.campsite_name
  }


  pub fn set_resource_link(&mut self, resource_link: String) {
    self.resource_link = resource_link;
  }

  pub fn with_resource_link(mut self, resource_link: String) -> FacilityCampsite {
    self.resource_link = resource_link;
    self
  }

  pub fn resource_link(&self) -> &String {
    &self.resource_link
  }


}



