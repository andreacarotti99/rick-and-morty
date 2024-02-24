use serde::{Deserialize, Serialize};
use crate::models::Info;


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

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationCharacter {
    name: String,
    url: String,
}

// Response for all characters and a single page
#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersResponse {
    info: Info,
    results: Vec<Character>,
}

pub type CharactersListResponse = Vec<Character>;

#[derive(Default, Serialize)]
pub struct FilterCharacter {
    pub name: Option<String>,
    pub status: Option<String>,
    pub species: Option<String>,
    #[serde(rename = "type")]
    pub character_type: Option<String>,
    pub gender: Option<String>,
}

