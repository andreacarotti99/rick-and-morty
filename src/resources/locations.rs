use serde::{Deserialize, Serialize};
use crate::models::Info;
use chrono::{DateTime, Utc};

/// Represents a location with its detailed attributes.
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

/// Response structure for fetching locations, including pagination information.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationResponse {
    info: Info,
    results: Vec<Location>,
}

/// Type alias for a list of locations, used in bulk location responses.
pub type LocationListResponse = Vec<Location>;

/// Filters for querying locations, allowing for partial matching on various fields.
#[derive(Default, Serialize)]
pub struct FilterLocation {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub location_type: Option<String>,
    pub dimension: Option<String>,
}

