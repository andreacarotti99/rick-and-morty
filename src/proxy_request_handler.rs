use serde_json::Value;
use crate::proxy::{Cache, Users};

/// Handles incoming API requests, checking the cache and user permissions before proceeding.
///
/// This function acts as a middleware for the proxy server. It validates the provided API key,
/// checks if the requested data is in the cache, and fetches from the remote API if necessary.
///
/// # Arguments
/// * `endpoint` - The specific API endpoint that the request is targeting.
/// * `api_key` - The API key provided by the user to authenticate the request.
/// * `cache` - A shared cache structure to store and retrieve cached API responses.
/// * `users` - A shared user structure to validate API keys.
///
/// # Returns
/// A `Result` with a `warp::Reply` if the request is successful, or a `warp::Rejection` if
/// the request fails due to invalid API key or if the data cannot be fetched from the API.
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
        println!("Proxy does have the resource in Cache!");
        return Ok(warp::reply::json(cached_response));
    } else {
        // Fetch the resource from the API if not in the cache
        let base = "https://rickandmortyapi.com/api/";
        let requested_url = format!("{}{}", base, endpoint);
        println!("Proxy doesn't have the resource... fetching it...");
        println!("Request URL: {}", requested_url);

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
