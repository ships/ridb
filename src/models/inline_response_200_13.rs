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
pub struct InlineResponse20013 {
  #[serde(rename = "RECDATA")]
  RECDATA: Option<Vec<::models::Media>>
}

impl InlineResponse20013 {
  pub fn new() -> InlineResponse20013 {
    InlineResponse20013 {
      RECDATA: None
    }
  }

  pub fn set_RECDATA(&mut self, RECDATA: Vec<::models::Media>) {
    self.RECDATA = Some(RECDATA);
  }

  pub fn with_RECDATA(mut self, RECDATA: Vec<::models::Media>) -> InlineResponse20013 {
    self.RECDATA = Some(RECDATA);
    self
  }

  pub fn RECDATA(&self) -> Option<&Vec<::models::Media>> {
    self.RECDATA.as_ref()
  }

  pub fn reset_RECDATA(&mut self) {
    self.RECDATA = None;
  }

}



