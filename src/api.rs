
use reqwest::{Error, Client};
use serde::de::DeserializeOwned;
use crate::resources::characters::{Character, CharactersResponse, FilterCharacter, CharactersListResponse};
use crate::resources::episodes::{Episode, EpisodeListResponse,FilterEpisode,  EpisodeResponse};
use crate::resources::locations::{Location, LocationResponse, FilterLocation, LocationListResponse};
use crate::models::Filter;
use crate::handle_calls::Handler;


pub async fn fetch_and_deserialize<T: DeserializeOwned>(url: &str, api_key: &str) -> Result<T, Error> {
    let client = Client::new();
    let response = client.get(url)
        .header("x-api-key", api_key)
        .send()
        .await?;

    response.json::<T>().await
}


// Characters

pub async fn fetch_filtered_characters(handler: &Handler, filter: &FilterCharacter) -> Result<CharactersResponse, Error> {
    let query_string = filter.to_query_string();
    let filtered_characters_url = format!("{}character/?{}", handler.base_url, query_string);
    println!("{}", filtered_characters_url);
    fetch_and_deserialize::<CharactersResponse>(&filtered_characters_url, &handler.api_key).await
}

pub async fn fetch_all_characters(handler: &Handler,) -> Result<CharactersResponse, Error> {
    let all_characters_url = format!("{}character", handler.base_url);
    fetch_and_deserialize::<CharactersResponse>(&all_characters_url, &handler.api_key).await
}


pub async fn fetch_single_character(handler: &Handler, character_id: i32) -> Result<Character, Error> {
    let single_character_url = format!("{}character/{}", handler.base_url, character_id);
    fetch_and_deserialize::<Character>(&single_character_url, &handler.api_key).await
}

pub async fn fetch_characters_list(handler: &Handler, character_ids: &[i32]) -> Result<CharactersListResponse, Error> {
    let ids_string = character_ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let list_characters_url = format!("{}character/{}", handler.base_url, ids_string);
    fetch_and_deserialize::<CharactersListResponse>(&list_characters_url, &handler.api_key).await
}

// locations 

pub async fn fetch_single_location(handler: &Handler, location_id: i32) -> Result<Location, Error> {
    let single_location_url = format!("{}location/{}", handler.base_url, location_id);
    fetch_and_deserialize::<Location>(&single_location_url, &handler.api_key).await
}

pub async fn fetch_all_locations(handler: &Handler) -> Result<LocationResponse, Error> {
    let all_locations_url = format!("{}location", handler.base_url);
    fetch_and_deserialize::<LocationResponse>(&all_locations_url, &handler.api_key).await
}

pub async fn fetch_filtered_locations(handler: &Handler, filter: &FilterLocation) -> Result<LocationResponse, Error> {
    let query_string = filter.to_query_string();
    let filtered_location_url = format!("{}location/?{}", handler.base_url, query_string);
    println!("{}", filtered_location_url);
    fetch_and_deserialize::<LocationResponse>(&filtered_location_url, &handler.api_key).await
}

pub async fn fetch_location_list(handler: &Handler, location_ids: &[i32]) -> Result<LocationListResponse, Error> {
    let ids_string = location_ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let list_location_url = format!("{}location/{}", handler.base_url, ids_string);
    fetch_and_deserialize::<LocationListResponse>(&list_location_url, &handler.api_key).await
}

// episodes

pub async fn fetch_single_episode(handler: &Handler, episode_id: i32) -> Result<Episode, Error> {
    let single_episde_url = format!("{}episode/{}", handler.base_url, episode_id);
    fetch_and_deserialize::<Episode>(&single_episde_url, &handler.api_key).await
}

pub async fn fetch_all_episodes(handler: &Handler) -> Result<EpisodeResponse, Error> {
    let all_episode_url = format!("{}episode/", handler.base_url);
    print!("{}\n", all_episode_url);
    fetch_and_deserialize::<EpisodeResponse>(&all_episode_url, &handler.api_key).await
}

pub async fn fetch_filtered_episode(handler: &Handler, filter: &FilterEpisode) -> Result<EpisodeResponse, Error> {
    let query_string = filter.to_query_string();
    let filtered_episode_url = format!("{}episode/?{}", handler.base_url, query_string);
    println!("{}", filtered_episode_url);
    fetch_and_deserialize::<EpisodeResponse>(&filtered_episode_url, &handler.api_key).await
}

pub async fn fetch_episode_list(handler: &Handler, episode_ids: &[i32]) -> Result<EpisodeListResponse, Error> {
    let ids_string = episode_ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let list_episode_url = format!("{}episode/{}", handler.base_url, ids_string);
    fetch_and_deserialize::<EpisodeListResponse>(&list_episode_url, &handler.api_key).await
}