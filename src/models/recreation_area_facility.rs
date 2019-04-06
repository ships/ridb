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
pub struct RecreationAreaFacility {
  #[serde(rename = "FacilityID")]
  facility_id: String,
  #[serde(rename = "FacilityName")]
  facility_name: String
}

impl RecreationAreaFacility {
  pub fn new(facility_id: String, facility_name: String) -> RecreationAreaFacility {
    RecreationAreaFacility {
      facility_id: facility_id,
      facility_name: facility_name
    }
  }

  pub fn set_facility_id(&mut self, facility_id: String) {
    self.facility_id = facility_id;
  }

  pub fn with_facility_id(mut self, facility_id: String) -> RecreationAreaFacility {
    self.facility_id = facility_id;
    self
  }

  pub fn facility_id(&self) -> &String {
    &self.facility_id
  }


  pub fn set_facility_name(&mut self, facility_name: String) {
    self.facility_name = facility_name;
  }

  pub fn with_facility_name(mut self, facility_name: String) -> RecreationAreaFacility {
    self.facility_name = facility_name;
    self
  }

  pub fn facility_name(&self) -> &String {
    &self.facility_name
  }


}



