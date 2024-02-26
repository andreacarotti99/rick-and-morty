use serde_json::Value;
use crate::proxy::{Cache, Users};

// for the first implementation for a user is enough to have a valid api key. not his api key
pub async fn request_handler(endpoint: String, api_key: String, cache: Cache, users: Users) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Received request for {}", endpoint);

    // Check API key validity
    let users = users.lock().await;
    
    let api_key_exists = users.contains_key(&api_key);
    
    if !api_key_exists {
        println!("API key is either wrong or not provided");
        println!("API key provided: {}", api_key);
        return Err(warp::reject::not_found());
    }

    // Check if the resource is in the cache
    let mut cache = cache.lock().await;
    if let Some(cached_response) = cache.get(&endpoint) {
        println!("Proxy does have the resource in Cache!\n");
        return Ok(warp::reply::json(cached_response));
    } else {
        // Fetch the resource from the API if not in the cache
        let base = "https://rickandmortyapi.com/api/";
        let requested_url = format!("{}{}", base, endpoint);
        println!("Proxy doesn't have the resource... fetching it...\n");
        println!("Request URL: {}\n", requested_url);

        let response = reqwest::get(&requested_url)
            .await
            .map_err(|_| warp::reject::not_found())?;
        let fetched_data: Value = response
            .json()
            .await
            .map_err(|_| warp::reject::not_found())?;

        // Insert the fetched data into the cache and return it
        cache.insert(endpoint.clone(), fetched_data.clone());
        Ok(warp::reply::json(&fetched_data))
    }
}
