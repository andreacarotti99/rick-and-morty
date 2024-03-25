use serde::{Deserialize, Serialize};
use crate::models::Info;

/// Represents a character with detailed information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    id: i32,
    name: String,
    status: String,
    species: String,
    #[serde(rename = "type")]
    character_type: String,
    gender: String,
    origin: LocationCharacter,
    location: LocationCharacter,
    image: String,
    episode: Vec<String>,
    url: String,
    created: String,
}

/// Represents a location as it relates to a character's origin or current place.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationCharacter {
    name: String,
    url: String,
}

/// Response structure used for fetching all characters or a page of characters.
#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersResponse {
    info: Info,
    results: Vec<Character>,
}

/// Type alias for a list of characters, used in bulk character responses.
pub type CharactersListResponse = Vec<Character>;

/// Filters for querying characters, supporting partial matching on various fields.
#[derive(Default, Serialize)]
pub struct FilterCharacter {
    pub name: Option<String>,
    pub status: Option<String>,
    pub species: Option<String>,
    #[serde(rename = "type")]
    pub character_type: Option<String>,
    pub gender: Option<String>,
}

