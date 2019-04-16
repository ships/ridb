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
pub struct Tour {
  #[serde(rename = "TourID")]
  tour_id: String,
  #[serde(rename = "FacilityID")]
  facility_id: String,
  #[serde(rename = "TourName")]
  tour_name: String,
  #[serde(rename = "TourType")]
  tour_type: String,
  #[serde(rename = "TourDescription")]
  tour_description: String,
  #[serde(rename = "TourDuration")]
  tour_duration: i32,
  #[serde(rename = "TourAccessible")]
  tour_accessible: bool,
  #[serde(rename = "CreatedDate")]
  created_date: String,
  #[serde(rename = "LastUpdatedDate")]
  last_updated_date: String,
  #[serde(rename = "ATTRIBUTES")]
  ATTRIBUTES: Vec<::models::Attribute>,
  #[serde(rename = "ENTITYMEDIA")]
  ENTITYMEDIA: Vec<::models::Media>,
  #[serde(rename = "MEMBERTOURS")]
  MEMBERTOURS: Vec<::models::TourMembertours>
}

impl Tour {
  pub fn new(tour_id: String, facility_id: String, tour_name: String, tour_type: String, tour_description: String, tour_duration: i32, tour_accessible: bool, created_date: String, last_updated_date: String, ATTRIBUTES: Vec<::models::Attribute>, ENTITYMEDIA: Vec<::models::Media>, MEMBERTOURS: Vec<::models::TourMembertours>) -> Tour {
    Tour {
      tour_id: tour_id,
      facility_id: facility_id,
      tour_name: tour_name,
      tour_type: tour_type,
      tour_description: tour_description,
      tour_duration: tour_duration,
      tour_accessible: tour_accessible,
      created_date: created_date,
      last_updated_date: last_updated_date,
      ATTRIBUTES: ATTRIBUTES,
      ENTITYMEDIA: ENTITYMEDIA,
      MEMBERTOURS: MEMBERTOURS
    }
  }

  pub fn set_tour_id(&mut self, tour_id: String) {
    self.tour_id = tour_id;
  }

  pub fn with_tour_id(mut self, tour_id: String) -> Tour {
    self.tour_id = tour_id;
    self
  }

  pub fn tour_id(&self) -> &String {
    &self.tour_id
  }


  pub fn set_facility_id(&mut self, facility_id: String) {
    self.facility_id = facility_id;
  }

  pub fn with_facility_id(mut self, facility_id: String) -> Tour {
    self.facility_id = facility_id;
    self
  }

  pub fn facility_id(&self) -> &String {
    &self.facility_id
  }


  pub fn set_tour_name(&mut self, tour_name: String) {
    self.tour_name = tour_name;
  }

  pub fn with_tour_name(mut self, tour_name: String) -> Tour {
    self.tour_name = tour_name;
    self
  }

  pub fn tour_name(&self) -> &String {
    &self.tour_name
  }


  pub fn set_tour_type(&mut self, tour_type: String) {
    self.tour_type = tour_type;
  }

  pub fn with_tour_type(mut self, tour_type: String) -> Tour {
    self.tour_type = tour_type;
    self
  }

  pub fn tour_type(&self) -> &String {
    &self.tour_type
  }


  pub fn set_tour_description(&mut self, tour_description: String) {
    self.tour_description = tour_description;
  }

  pub fn with_tour_description(mut self, tour_description: String) -> Tour {
    self.tour_description = tour_description;
    self
  }

  pub fn tour_description(&self) -> &String {
    &self.tour_description
  }


  pub fn set_tour_duration(&mut self, tour_duration: i32) {
    self.tour_duration = tour_duration;
  }

  pub fn with_tour_duration(mut self, tour_duration: i32) -> Tour {
    self.tour_duration = tour_duration;
    self
  }

  pub fn tour_duration(&self) -> &i32 {
    &self.tour_duration
  }


  pub fn set_tour_accessible(&mut self, tour_accessible: bool) {
    self.tour_accessible = tour_accessible;
  }

  pub fn with_tour_accessible(mut self, tour_accessible: bool) -> Tour {
    self.tour_accessible = tour_accessible;
    self
  }

  pub fn tour_accessible(&self) -> &bool {
    &self.tour_accessible
  }


  pub fn set_created_date(&mut self, created_date: String) {
    self.created_date = created_date;
  }

  pub fn with_created_date(mut self, created_date: String) -> Tour {
    self.created_date = created_date;
    self
  }

  pub fn created_date(&self) -> &String {
    &self.created_date
  }


  pub fn set_last_updated_date(&mut self, last_updated_date: String) {
    self.last_updated_date = last_updated_date;
  }

  pub fn with_last_updated_date(mut self, last_updated_date: String) -> Tour {
    self.last_updated_date = last_updated_date;
    self
  }

  pub fn last_updated_date(&self) -> &String {
    &self.last_updated_date
  }


  pub fn set_ATTRIBUTES(&mut self, ATTRIBUTES: Vec<::models::Attribute>) {
    self.ATTRIBUTES = ATTRIBUTES;
  }

  pub fn with_ATTRIBUTES(mut self, ATTRIBUTES: Vec<::models::Attribute>) -> Tour {
    self.ATTRIBUTES = ATTRIBUTES;
    self
  }

  pub fn ATTRIBUTES(&self) -> &Vec<::models::Attribute> {
    &self.ATTRIBUTES
  }


  pub fn set_ENTITYMEDIA(&mut self, ENTITYMEDIA: Vec<::models::Media>) {
    self.ENTITYMEDIA = ENTITYMEDIA;
  }

  pub fn with_ENTITYMEDIA(mut self, ENTITYMEDIA: Vec<::models::Media>) -> Tour {
    self.ENTITYMEDIA = ENTITYMEDIA;
    self
  }

  pub fn ENTITYMEDIA(&self) -> &Vec<::models::Media> {
    &self.ENTITYMEDIA
  }


  pub fn set_MEMBERTOURS(&mut self, MEMBERTOURS: Vec<::models::TourMembertours>) {
    self.MEMBERTOURS = MEMBERTOURS;
  }

  pub fn with_MEMBERTOURS(mut self, MEMBERTOURS: Vec<::models::TourMembertours>) -> Tour {
    self.MEMBERTOURS = MEMBERTOURS;
    self
  }

  pub fn MEMBERTOURS(&self) -> &Vec<::models::TourMembertours> {
    &self.MEMBERTOURS
  }


}



