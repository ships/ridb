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
pub struct InlineResponse200 {
  #[serde(rename = "RECDATA")]
  RECDATA: Option<Vec<::models::Organization>>,
  #[serde(rename = "METADATA")]
  METADATA: Option<::models::InlineResponse200Metadata>
}

impl InlineResponse200 {
  pub fn new() -> InlineResponse200 {
    InlineResponse200 {
      RECDATA: None,
      METADATA: None
    }
  }

  pub fn set_RECDATA(&mut self, RECDATA: Vec<::models::Organization>) {
    self.RECDATA = Some(RECDATA);
  }

  pub fn with_RECDATA(mut self, RECDATA: Vec<::models::Organization>) -> InlineResponse200 {
    self.RECDATA = Some(RECDATA);
    self
  }

  pub fn RECDATA(&self) -> Option<&Vec<::models::Organization>> {
    self.RECDATA.as_ref()
  }

  pub fn reset_RECDATA(&mut self) {
    self.RECDATA = None;
  }

  pub fn set_METADATA(&mut self, METADATA: ::models::InlineResponse200Metadata) {
    self.METADATA = Some(METADATA);
  }

  pub fn with_METADATA(mut self, METADATA: ::models::InlineResponse200Metadata) -> InlineResponse200 {
    self.METADATA = Some(METADATA);
    self
  }

  pub fn METADATA(&self) -> Option<&::models::InlineResponse200Metadata> {
    self.METADATA.as_ref()
  }

  pub fn reset_METADATA(&mut self) {
    self.METADATA = None;
  }

}



