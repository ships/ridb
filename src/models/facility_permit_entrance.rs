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
pub struct FacilityPermitEntrance {
  #[serde(rename = "PermitEntranceID")]
  permit_entrance_id: String,
  #[serde(rename = "PermitEntranceName")]
  permit_entrance_name: String
}

impl FacilityPermitEntrance {
  pub fn new(permit_entrance_id: String, permit_entrance_name: String) -> FacilityPermitEntrance {
    FacilityPermitEntrance {
      permit_entrance_id: permit_entrance_id,
      permit_entrance_name: permit_entrance_name
    }
  }

  pub fn set_permit_entrance_id(&mut self, permit_entrance_id: String) {
    self.permit_entrance_id = permit_entrance_id;
  }

  pub fn with_permit_entrance_id(mut self, permit_entrance_id: String) -> FacilityPermitEntrance {
    self.permit_entrance_id = permit_entrance_id;
    self
  }

  pub fn permit_entrance_id(&self) -> &String {
    &self.permit_entrance_id
  }


  pub fn set_permit_entrance_name(&mut self, permit_entrance_name: String) {
    self.permit_entrance_name = permit_entrance_name;
  }

  pub fn with_permit_entrance_name(mut self, permit_entrance_name: String) -> FacilityPermitEntrance {
    self.permit_entrance_name = permit_entrance_name;
    self
  }

  pub fn permit_entrance_name(&self) -> &String {
    &self.permit_entrance_name
  }


}



