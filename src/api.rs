
use reqwest::{Error, Client};
use serde::de::DeserializeOwned;
use crate::resources::characters::{Character, CharactersResponse, FilterCharacter, CharactersListResponse};
use crate::resources::episodes::{Episode, EpisodeListResponse,FilterEpisode,  EpisodeResponse};
use crate::resources::locations::{Location, LocationResponse, FilterLocation, LocationListResponse};
use crate::models::Filter;
use crate::handle_calls::Handler;
use crate::proxy::SignupInfo;
//use crate::proxy::start_proxy;




pub async fn fetch_and_deserialize<T: DeserializeOwned>(url: &str, api_key: &str) -> Result<T, Error> {
    //println!("{}", url);
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
    println!("{}", query_string);
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
    println!("{}",single_character_url);
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

// signup
pub async fn send_signup(requested_username: &str) -> Result<String, Error> {
    
    let signup_info = SignupInfo {
        username: requested_username.to_string()
    };
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3030/signup")
        .json(&signup_info) 
        .send()
        .await?;

    let body = res.text().await?;
    
    Ok(body)
}


#[tokio::test]
async fn test_proxy_server_concurrency_signup() {
    // testing 1000 different clients (username) requesting 1000 different API keys

    tokio::spawn(async {
        crate::proxy::start_proxy().await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let client_count = 1000; 
    let mut tasks = Vec::new();
    let start = tokio::time::Instant::now();
    
    for i in 0..client_count {
        let username = format!("andrea{}", i); 
        tasks.push(tokio::spawn(async move { 
            match send_signup(&username).await {
                Ok(response) => println!("{}", response),
                Err(e) => eprintln!("Error signing up: {}", e),
            }
        }));
    }

    let _results: Vec<_> = futures::future::join_all(tasks).await;
    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}

// testing 100 requests to the server 

/* 
#[tokio::test]
async fn test_proxy_server_concurrency_requests() {
    // to test this it is better to run the server separately since the stdout interfere
    /* 
    tokio::spawn(async {
        crate::proxy::start_proxy().await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    */
    
    // speed test for the server results
    
    let client_count = 100;
    let mut tasks = Vec::new();
    let start = tokio::time::Instant::now();

    for i in 0..client_count {
        let handler = Handler {
            base_url: "http://rickandmortyapi.com/api/".to_string(),
            api_key: "".to_string()
        };
        
        tasks.push(tokio::spawn(async move { 
            handler.handle_fetch_single_character((i + 1).to_string()).await;
        }));
    }

    let _results: Vec<_> = futures::future::join_all(tasks).await;
    let duration_server_requests = start.elapsed();

    // now test the cached results
    let client_count = 100;
    let mut tasks = Vec::new();
    let start = tokio::time::Instant::now();

    for i in 0..client_count {
        let handler = Handler {
            base_url: "http://127.0.0.1:3030/proxy/".to_string(),
            api_key: "secret_key".to_string()
        };
        
        tasks.push(tokio::spawn(async move { 
            handler.handle_fetch_single_character((i + 1).to_string()).await;
        }));
    }

    let _results: Vec<_> = futures::future::join_all(tasks).await;
    let duration_cache_requests = start.elapsed();

    println!("Total execution time cache: {:?}", duration_cache_requests);
    println!("Total execution time server: {:?}", duration_server_requests);

}
*/
