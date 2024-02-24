use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, reject, http::StatusCode, Reply, Rejection, reply, body};
use serde::Deserialize;


type Users = Arc<Mutex<HashMap<String, String>>>;
type Cache = Arc<Mutex<HashMap<String, String>>>;

#[derive(Deserialize)]
struct SignupInfo {
    username: String,
}

pub async fn start_proxy() {
    let cache = Arc::new(Mutex::new(HashMap::new())); // Shared cache
    let users = Arc::new(Mutex::new(HashMap::new())); // Simulated user storage

    let signup = warp::path("signup")
        .and(warp::post())
        .and(body::json::<SignupInfo>())  // Extract the username from the request body
        .and_then({
            let users_clone = users.clone(); 
            move |signup_info: SignupInfo| {
                signup_handler(users_clone.clone(), signup_info.username) // Pass the extracted username to the handler
            }
        });



    let proxy = warp::path("proxy")
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::header::<String>("x-api-key"))
        .and_then({
            let cache_clone = cache.clone(); 
            let users_clone = users.clone(); 
            move |endpoint: String, api_key: String| {
                proxy_handler(endpoint, api_key, cache_clone.clone(), users_clone.clone()) // Clone for handler
            }
        });

    let routes = signup.or(proxy);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn signup_handler(users: Users, username: String) -> Result<impl Reply, Rejection> {
    let mut users = users.lock().await; // Acquiring the lock on users

    if users.contains_key(&username) {
        let error_message = reply::json(&"Username already exists");
        let with_status = reply::with_status(error_message, StatusCode::CONFLICT);
        return Ok(with_status);
    }

    
    let api_key = format!("{}_{}", username, users.len() + 1);
    users.insert(username, api_key.clone());

    let json_reply_api_key = reply::json(&api_key);
    let with_status = reply::with_status(json_reply_api_key, StatusCode::OK);
    Ok(with_status)
}

async fn proxy_handler(
    endpoint: String,
    api_key: String,
    cache: Cache,
    users: Users,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Simplified proxy logic for demonstration
    let users = users.lock().await;
    if !users.contains_key(&api_key) {
        // Reject if API key is invalid
        return Err(reject::not_found());
    }

    let mut cache = cache.lock().await;
    if let Some(response) = cache.get(&endpoint) {
        // Return cached response if available
        return Ok(warp::reply::json(response));
    }

    // Simulate fetching data and caching it
    let fetched_data = format!("Fetched data for endpoint: {}", endpoint);
    cache.insert(endpoint, fetched_data.clone());

    Ok(warp::reply::json(&fetched_data))
}

