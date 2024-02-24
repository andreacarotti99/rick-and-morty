use serde_json::Value;
use warp::reject;
use crate::proxy::{Cache, Users};

// for the first implementation for a user is enough to have a valid api key. not him api key
pub async fn proxy_handler(endpoint: String, api_key: String, cache: Cache, users: Users) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Received request for {}", endpoint);
    let users = users.lock().await;
    let api_key_exists = users.values().any(|key| key == &api_key);

    if !api_key_exists {
        println!("API key is either wrong or not provided");
        println!("API key provided: {}", api_key);
        // Printing all API keys for debugging (be cautious with sensitive info)
        println!("List of all API keys:");
        for (user, key) in users.iter() {
            println!("User: {}, API Key: {}", user, key);
        }

        return Err(reject::not_found());
    }
    // the api key is valid
    //checking if the resource is in cache
    let mut cache = cache.lock().await;
    if let Some(response) = cache.get(&endpoint) {
        println!("Proxy does have the resource in Cache!\n");
        return Ok(warp::reply::json(response));
    }
    else {
        //otherwise fetch the resource from the api
        let base = "https://rickandmortyapi.com/api/";
        let requested_url = format!("{}{}", base, endpoint);
        println!("Proxy doesn't have the resource... fetching it...\n");
        println!("Request URL: {}\n", requested_url);

        let response = reqwest::get(&requested_url).await.map_err(|_| warp::reject::not_found())?;
        let fetched_data: Value = response.json().await.map_err(|_| warp::reject::not_found())?;

        cache.insert(endpoint, fetched_data.to_string().clone());
        Ok(warp::reply::json(&fetched_data))


    }
}

