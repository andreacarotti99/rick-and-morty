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
    origin: Location,
    location: Location,
    image: String,
    episode: Vec<String>,
    url: String,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
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
pub struct Filter {
    pub name: Option<String>,
    pub status: Option<String>,
    pub species: Option<String>,
    #[serde(rename = "type")]
    pub character_type: Option<String>,
    pub gender: Option<String>,
}



impl Filter {
    pub fn to_query_string(&self) -> String {
        // Use serde_qs to serialize the Filter struct into a query string
        let query = serde_qs::to_string(self).unwrap_or_default();
        println!("{}", query);
        query
    }
}