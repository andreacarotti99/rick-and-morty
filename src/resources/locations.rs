use serde::{Deserialize, Serialize};
use crate::models::Info;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String, 
    pub dimension: String,
    pub residents: Vec<String>, 
    pub url: String,
    pub created: DateTime<Utc>, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationResponse {
    info: Info,
    results: Vec<Location>,
}

pub type LocationListResponse = Vec<Location>;

#[derive(Default, Serialize)]
pub struct FilterLocation {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub location_type: Option<String>,
    pub dimension: Option<String>,
}

