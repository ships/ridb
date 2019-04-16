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
pub struct FacilityRecArea {
  #[serde(rename = "RecAreaID")]
  rec_area_id: String,
  #[serde(rename = "RecAreaName")]
  rec_area_name: String,
  #[serde(rename = "ResourceLink")]
  resource_link: String
}

impl FacilityRecArea {
  pub fn new(rec_area_id: String, rec_area_name: String, resource_link: String) -> FacilityRecArea {
    FacilityRecArea {
      rec_area_id: rec_area_id,
      rec_area_name: rec_area_name,
      resource_link: resource_link
    }
  }

  pub fn set_rec_area_id(&mut self, rec_area_id: String) {
    self.rec_area_id = rec_area_id;
  }

  pub fn with_rec_area_id(mut self, rec_area_id: String) -> FacilityRecArea {
    self.rec_area_id = rec_area_id;
    self
  }

  pub fn rec_area_id(&self) -> &String {
    &self.rec_area_id
  }


  pub fn set_rec_area_name(&mut self, rec_area_name: String) {
    self.rec_area_name = rec_area_name;
  }

  pub fn with_rec_area_name(mut self, rec_area_name: String) -> FacilityRecArea {
    self.rec_area_name = rec_area_name;
    self
  }

  pub fn rec_area_name(&self) -> &String {
    &self.rec_area_name
  }


  pub fn set_resource_link(&mut self, resource_link: String) {
    self.resource_link = resource_link;
  }

  pub fn with_resource_link(mut self, resource_link: String) -> FacilityRecArea {
    self.resource_link = resource_link;
    self
  }

  pub fn resource_link(&self) -> &String {
    &self.resource_link
  }


}



