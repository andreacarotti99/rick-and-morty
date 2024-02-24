use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, reject, http::StatusCode, Reply, Rejection, reply, body};
use serde::Deserialize;
use serde_json::Value;


type Users = Arc<Mutex<HashMap<String, String>>>;
type Cache = Arc<Mutex<HashMap<String, String>>>;

#[derive(Deserialize)]
struct SignupInfo {
    username: String,
}

pub async fn start_proxy() {
    //the cache contains the string requested and the responde
    let cache = Arc::new(Mutex::new(HashMap::new())); // Shared cache

    // the user storage contains the user with its api key
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
        println!("username: {} already exists! signup failed", username);
        return Ok(with_status);
    }

    
    let api_key = format!("{}_{}", username, users.len() + 1);
    
    users.insert(username.clone(), api_key.clone());

    let json_reply_api_key = reply::json(&api_key);
    let with_status = reply::with_status(json_reply_api_key, StatusCode::OK);
    println!("new signup! username: {}, api-key: {}", username, api_key);
    Ok(with_status)
}

// for the first implementation for a user is enough to have a valid api key. not him api key
async fn proxy_handler(endpoint: String, api_key: String, cache: Cache, users: Users) -> Result<impl warp::Reply, warp::Rejection> {
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

