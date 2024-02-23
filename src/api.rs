
use reqwest::Error;
use serde::de::DeserializeOwned;
use crate::models::{Character, CharactersResponse, Filter, CharactersListResponse};


const BASE_URL: &str = "https://rickandmortyapi.com/api/";


pub async fn fetch_and_deserialize<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    reqwest::get(url).await?.json::<T>().await
}


pub async fn fetch_filtered_characters(filter: &Filter) -> Result<CharactersResponse, Error> {
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