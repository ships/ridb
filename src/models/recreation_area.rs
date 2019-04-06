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
pub struct RecreationArea {
  #[serde(rename = "RecAreaID")]
  rec_area_id: String,
  #[serde(rename = "OrgRecAreaID")]
  org_rec_area_id: String,
  #[serde(rename = "ParentOrgID")]
  parent_org_id: Option<String>,
  #[serde(rename = "RecAreaName")]
  rec_area_name: String,
  #[serde(rename = "RecAreaDescription")]
  rec_area_description: String,
  #[serde(rename = "RecAreaFeeDescription")]
  rec_area_fee_description: String,
  #[serde(rename = "RecAreaDirections")]
  rec_area_directions: String,
  #[serde(rename = "RecAreaPhone")]
  rec_area_phone: String,
  #[serde(rename = "RecAreaEmail")]
  rec_area_email: String,
  #[serde(rename = "RecAreaReservationURL")]
  rec_area_reservation_url: String,
  #[serde(rename = "RecAreaMapURL")]
  rec_area_map_url: String,
  #[serde(rename = "GEOJSON")]
  GEOJSON: Value,
  #[serde(rename = "RecAreaLongitude")]
  rec_area_longitude: f64,
  #[serde(rename = "RecAreaLatitude")]
  rec_area_latitude: f64,
  #[serde(rename = "StayLimit")]
  stay_limit: String,
  #[serde(rename = "Keywords")]
  keywords: String,
  #[serde(rename = "ORGANIZATION")]
  ORGANIZATION: Option<Vec<::models::Organization>>,
  #[serde(rename = "FACILITY")]
  FACILITY: Option<Vec<::models::RecreationAreaFacility>>,
  #[serde(rename = "RECAREAADDRESS")]
  RECAREAADDRESS: Option<Vec<::models::RecreationAreaAddress>>,
  #[serde(rename = "ACTIVITY")]
  ACTIVITY: Option<Vec<::models::RecreationAreaActivity>>,
  #[serde(rename = "EVENT")]
  EVENT: Option<Vec<::models::Event>>,
  #[serde(rename = "MEDIA")]
  MEDIA: Option<Vec<::models::Media>>,
  #[serde(rename = "LINK")]
  LINK: Option<Vec<::models::Link>>
}

impl RecreationArea {
  pub fn new(rec_area_id: String, org_rec_area_id: String, rec_area_name: String, rec_area_description: String, rec_area_fee_description: String, rec_area_directions: String, rec_area_phone: String, rec_area_email: String, rec_area_reservation_url: String, rec_area_map_url: String, GEOJSON: Value, rec_area_longitude: f64, rec_area_latitude: f64, stay_limit: String, keywords: String) -> RecreationArea {
    RecreationArea {
      rec_area_id: rec_area_id,
      org_rec_area_id: org_rec_area_id,
      parent_org_id: None,
      rec_area_name: rec_area_name,
      rec_area_description: rec_area_description,
      rec_area_fee_description: rec_area_fee_description,
      rec_area_directions: rec_area_directions,
      rec_area_phone: rec_area_phone,
      rec_area_email: rec_area_email,
      rec_area_reservation_url: rec_area_reservation_url,
      rec_area_map_url: rec_area_map_url,
      GEOJSON: GEOJSON,
      rec_area_longitude: rec_area_longitude,
      rec_area_latitude: rec_area_latitude,
      stay_limit: stay_limit,
      keywords: keywords,
      ORGANIZATION: None,
      FACILITY: None,
      RECAREAADDRESS: None,
      ACTIVITY: None,
      EVENT: None,
      MEDIA: None,
      LINK: None
    }
  }

  pub fn set_rec_area_id(&mut self, rec_area_id: String) {
    self.rec_area_id = rec_area_id;
  }

  pub fn with_rec_area_id(mut self, rec_area_id: String) -> RecreationArea {
    self.rec_area_id = rec_area_id;
    self
  }

  pub fn rec_area_id(&self) -> &String {
    &self.rec_area_id
  }


  pub fn set_org_rec_area_id(&mut self, org_rec_area_id: String) {
    self.org_rec_area_id = org_rec_area_id;
  }

  pub fn with_org_rec_area_id(mut self, org_rec_area_id: String) -> RecreationArea {
    self.org_rec_area_id = org_rec_area_id;
    self
  }

  pub fn org_rec_area_id(&self) -> &String {
    &self.org_rec_area_id
  }


  pub fn set_parent_org_id(&mut self, parent_org_id: String) {
    self.parent_org_id = Some(parent_org_id);
  }

  pub fn with_parent_org_id(mut self, parent_org_id: String) -> RecreationArea {
    self.parent_org_id = Some(parent_org_id);
    self
  }

  pub fn parent_org_id(&self) -> Option<&String> {
    self.parent_org_id.as_ref()
  }

  pub fn reset_parent_org_id(&mut self) {
    self.parent_org_id = None;
  }

  pub fn set_rec_area_name(&mut self, rec_area_name: String) {
    self.rec_area_name = rec_area_name;
  }

  pub fn with_rec_area_name(mut self, rec_area_name: String) -> RecreationArea {
    self.rec_area_name = rec_area_name;
    self
  }

  pub fn rec_area_name(&self) -> &String {
    &self.rec_area_name
  }


  pub fn set_rec_area_description(&mut self, rec_area_description: String) {
    self.rec_area_description = rec_area_description;
  }

  pub fn with_rec_area_description(mut self, rec_area_description: String) -> RecreationArea {
    self.rec_area_description = rec_area_description;
    self
  }

  pub fn rec_area_description(&self) -> &String {
    &self.rec_area_description
  }


  pub fn set_rec_area_fee_description(&mut self, rec_area_fee_description: String) {
    self.rec_area_fee_description = rec_area_fee_description;
  }

  pub fn with_rec_area_fee_description(mut self, rec_area_fee_description: String) -> RecreationArea {
    self.rec_area_fee_description = rec_area_fee_description;
    self
  }

  pub fn rec_area_fee_description(&self) -> &String {
    &self.rec_area_fee_description
  }


  pub fn set_rec_area_directions(&mut self, rec_area_directions: String) {
    self.rec_area_directions = rec_area_directions;
  }

  pub fn with_rec_area_directions(mut self, rec_area_directions: String) -> RecreationArea {
    self.rec_area_directions = rec_area_directions;
    self
  }

  pub fn rec_area_directions(&self) -> &String {
    &self.rec_area_directions
  }


  pub fn set_rec_area_phone(&mut self, rec_area_phone: String) {
    self.rec_area_phone = rec_area_phone;
  }

  pub fn with_rec_area_phone(mut self, rec_area_phone: String) -> RecreationArea {
    self.rec_area_phone = rec_area_phone;
    self
  }

  pub fn rec_area_phone(&self) -> &String {
    &self.rec_area_phone
  }


  pub fn set_rec_area_email(&mut self, rec_area_email: String) {
    self.rec_area_email = rec_area_email;
  }

  pub fn with_rec_area_email(mut self, rec_area_email: String) -> RecreationArea {
    self.rec_area_email = rec_area_email;
    self
  }

  pub fn rec_area_email(&self) -> &String {
    &self.rec_area_email
  }


  pub fn set_rec_area_reservation_url(&mut self, rec_area_reservation_url: String) {
    self.rec_area_reservation_url = rec_area_reservation_url;
  }

  pub fn with_rec_area_reservation_url(mut self, rec_area_reservation_url: String) -> RecreationArea {
    self.rec_area_reservation_url = rec_area_reservation_url;
    self
  }

  pub fn rec_area_reservation_url(&self) -> &String {
    &self.rec_area_reservation_url
  }


  pub fn set_rec_area_map_url(&mut self, rec_area_map_url: String) {
    self.rec_area_map_url = rec_area_map_url;
  }

  pub fn with_rec_area_map_url(mut self, rec_area_map_url: String) -> RecreationArea {
    self.rec_area_map_url = rec_area_map_url;
    self
  }

  pub fn rec_area_map_url(&self) -> &String {
    &self.rec_area_map_url
  }


  pub fn set_GEOJSON(&mut self, GEOJSON: Value) {
    self.GEOJSON = GEOJSON;
  }

  pub fn with_GEOJSON(mut self, GEOJSON: Value) -> RecreationArea {
    self.GEOJSON = GEOJSON;
    self
  }

  pub fn GEOJSON(&self) -> &Value {
    &self.GEOJSON
  }


  pub fn set_rec_area_longitude(&mut self, rec_area_longitude: f64) {
    self.rec_area_longitude = rec_area_longitude;
  }

  pub fn with_rec_area_longitude(mut self, rec_area_longitude: f64) -> RecreationArea {
    self.rec_area_longitude = rec_area_longitude;
    self
  }

  pub fn rec_area_longitude(&self) -> &f64 {
    &self.rec_area_longitude
  }


  pub fn set_rec_area_latitude(&mut self, rec_area_latitude: f64) {
    self.rec_area_latitude = rec_area_latitude;
  }

  pub fn with_rec_area_latitude(mut self, rec_area_latitude: f64) -> RecreationArea {
    self.rec_area_latitude = rec_area_latitude;
    self
  }

  pub fn rec_area_latitude(&self) -> &f64 {
    &self.rec_area_latitude
  }


  pub fn set_stay_limit(&mut self, stay_limit: String) {
    self.stay_limit = stay_limit;
  }

  pub fn with_stay_limit(mut self, stay_limit: String) -> RecreationArea {
    self.stay_limit = stay_limit;
    self
  }

  pub fn stay_limit(&self) -> &String {
    &self.stay_limit
  }


  pub fn set_keywords(&mut self, keywords: String) {
    self.keywords = keywords;
  }

  pub fn with_keywords(mut self, keywords: String) -> RecreationArea {
    self.keywords = keywords;
    self
  }

  pub fn keywords(&self) -> &String {
    &self.keywords
  }


  pub fn set_ORGANIZATION(&mut self, ORGANIZATION: Vec<::models::Organization>) {
    self.ORGANIZATION = Some(ORGANIZATION);
  }

  pub fn with_ORGANIZATION(mut self, ORGANIZATION: Vec<::models::Organization>) -> RecreationArea {
    self.ORGANIZATION = Some(ORGANIZATION);
    self
  }

  pub fn ORGANIZATION(&self) -> Option<&Vec<::models::Organization>> {
    self.ORGANIZATION.as_ref()
  }

  pub fn reset_ORGANIZATION(&mut self) {
    self.ORGANIZATION = None;
  }

  pub fn set_FACILITY(&mut self, FACILITY: Vec<::models::RecreationAreaFacility>) {
    self.FACILITY = Some(FACILITY);
  }

  pub fn with_FACILITY(mut self, FACILITY: Vec<::models::RecreationAreaFacility>) -> RecreationArea {
    self.FACILITY = Some(FACILITY);
    self
  }

  pub fn FACILITY(&self) -> Option<&Vec<::models::RecreationAreaFacility>> {
    self.FACILITY.as_ref()
  }

  pub fn reset_FACILITY(&mut self) {
    self.FACILITY = None;
  }

  pub fn set_RECAREAADDRESS(&mut self, RECAREAADDRESS: Vec<::models::RecreationAreaAddress>) {
    self.RECAREAADDRESS = Some(RECAREAADDRESS);
  }

  pub fn with_RECAREAADDRESS(mut self, RECAREAADDRESS: Vec<::models::RecreationAreaAddress>) -> RecreationArea {
    self.RECAREAADDRESS = Some(RECAREAADDRESS);
    self
  }

  pub fn RECAREAADDRESS(&self) -> Option<&Vec<::models::RecreationAreaAddress>> {
    self.RECAREAADDRESS.as_ref()
  }

  pub fn reset_RECAREAADDRESS(&mut self) {
    self.RECAREAADDRESS = None;
  }

  pub fn set_ACTIVITY(&mut self, ACTIVITY: Vec<::models::RecreationAreaActivity>) {
    self.ACTIVITY = Some(ACTIVITY);
  }

  pub fn with_ACTIVITY(mut self, ACTIVITY: Vec<::models::RecreationAreaActivity>) -> RecreationArea {
    self.ACTIVITY = Some(ACTIVITY);
    self
  }

  pub fn ACTIVITY(&self) -> Option<&Vec<::models::RecreationAreaActivity>> {
    self.ACTIVITY.as_ref()
  }

  pub fn reset_ACTIVITY(&mut self) {
    self.ACTIVITY = None;
  }

  pub fn set_EVENT(&mut self, EVENT: Vec<::models::Event>) {
    self.EVENT = Some(EVENT);
  }

  pub fn with_EVENT(mut self, EVENT: Vec<::models::Event>) -> RecreationArea {
    self.EVENT = Some(EVENT);
    self
  }

  pub fn EVENT(&self) -> Option<&Vec<::models::Event>> {
    self.EVENT.as_ref()
  }

  pub fn reset_EVENT(&mut self) {
    self.EVENT = None;
  }

  pub fn set_MEDIA(&mut self, MEDIA: Vec<::models::Media>) {
    self.MEDIA = Some(MEDIA);
  }

  pub fn with_MEDIA(mut self, MEDIA: Vec<::models::Media>) -> RecreationArea {
    self.MEDIA = Some(MEDIA);
    self
  }

  pub fn MEDIA(&self) -> Option<&Vec<::models::Media>> {
    self.MEDIA.as_ref()
  }

  pub fn reset_MEDIA(&mut self) {
    self.MEDIA = None;
  }

  pub fn set_LINK(&mut self, LINK: Vec<::models::Link>) {
    self.LINK = Some(LINK);
  }

  pub fn with_LINK(mut self, LINK: Vec<::models::Link>) -> RecreationArea {
    self.LINK = Some(LINK);
    self
  }

  pub fn LINK(&self) -> Option<&Vec<::models::Link>> {
    self.LINK.as_ref()
  }

  pub fn reset_LINK(&mut self) {
    self.LINK = None;
  }

}



