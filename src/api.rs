
use reqwest::Error;
use serde::de::DeserializeOwned;
use crate::resources::characters::{Character, CharactersResponse, FilterCharacter, CharactersListResponse};
use crate::resources::locations::{Location, LocationResponse, FilterLocation, LocationListResponse};

const BASE_URL: &str = "https://rickandmortyapi.com/api/";


pub async fn fetch_and_deserialize<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    reqwest::get(url).await?.json::<T>().await
}


// Characters

pub async fn fetch_filtered_characters(filter: &FilterCharacter) -> Result<CharactersResponse, Error> {
    let query_string = filter.to_query_string();
    let filtered_characters_url = format!("{}character/?{}", BASE_URL, query_string);
    println!("{}", filtered_characters_url);
    fetch_and_deserialize::<CharactersResponse>(&filtered_characters_url).await
}

pub async fn fetch_all_characters() -> Result<CharactersResponse, Error> {
    let all_characters_url = format!("{}character", BASE_URL);
    fetch_and_deserialize::<CharactersResponse>(&all_characters_url).await
}


pub async fn fetch_single_character(character_id: i32) -> Result<Character, Error> {
    let single_character_url = format!("{}character/{}", BASE_URL, character_id);
    fetch_and_deserialize::<Character>(&single_character_url).await
}

pub async fn fetch_characters_list(character_ids: &[i32]) -> Result<CharactersListResponse, Error> {
    let ids_string = character_ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let list_characters_url = format!("{}character/{}", BASE_URL, ids_string);
    fetch_and_deserialize::<CharactersListResponse>(&list_characters_url).await
}

// locations 

pub async fn fetch_single_location(location_id: i32) -> Result<Location, Error> {
    let single_location_url = format!("{}location/{}", BASE_URL, location_id);
    fetch_and_deserialize::<Location>(&single_location_url).await
}

pub async fn fetch_all_locations() -> Result<LocationResponse, Error> {
    let all_locations_url = format!("{}location", BASE_URL);
    fetch_and_deserialize::<LocationResponse>(&all_locations_url).await
}

pub async fn fetch_filtered_locations(filter: &FilterLocation) -> Result<LocationResponse, Error> {
    let query_string = filter.to_query_string();
    let filtered_location_url = format!("{}location/?{}", BASE_URL, query_string);
    println!("{}", filtered_location_url);
    fetch_and_deserialize::<LocationResponse>(&filtered_location_url).await
}

pub async fn fetch_location_list(location_ids: &[i32]) -> Result<LocationListResponse, Error> {
    let ids_string = location_ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let list_location_url = format!("{}location/{}", BASE_URL, ids_string);
    fetch_and_deserialize::<LocationListResponse>(&list_location_url).await
}
