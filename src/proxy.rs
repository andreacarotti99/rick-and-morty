use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, body};
use serde::{Deserialize, Serialize};
use crate::{proxy_signup, proxy_request_handler};
use serde_json::Value;

/// Shared storage type definitions.
pub type Users = Arc<Mutex<HashMap<String, String>>>;
pub type Cache = Arc<Mutex<HashMap<String, Value>>>;

/// Network constants for the proxy server.
const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 3030;

/// Data structure for signup information.
#[derive(Serialize, Deserialize)]
pub struct SignupInfo {
    pub username: String,
}

/// Starts the proxy server to handle requests.
///
/// Initializes the cache and user storage, sets up routing for signup and proxy requests, and starts
/// the Warp server to listen for incoming requests. It includes a backdoor for testing purposes.
pub async fn start_proxy() {

    //the cache contains the string requested and the responde
    let cache = Arc::new(Mutex::new(HashMap::new())); // Shared cache

    // the user storage contains the user with its api key
    let users = Arc::new(Mutex::new(HashMap::new())); // Simulated user storage

    //inserting an api backdoor for testing
    {
        let mut users_lock = users.lock().await;
        users_lock.insert("secret_key".to_string(), "secret_username".to_string());
    }

    // Define the signup route
    let signup = warp::path("signup")
        .and(warp::post())
        .and(body::json::<SignupInfo>())  // Extract the username from the request body
        .and_then({
            let users_clone = users.clone(); 
            move |signup_info: SignupInfo| {
                proxy_signup::signup_handler(users_clone.clone(), signup_info.username) // Pass the extracted username to the handler
            }
        });

    // Define the standard proxy route
    let proxy_std_requests = warp::path("proxy")
        .and(warp::get())
        .and(warp::path::tail())
        //.and(warp::query::raw())
        .and(warp::header::<String>("x-api-key"))
        .and_then({
            let cache_clone = cache.clone(); 
            let users_clone = users.clone(); 
            move |tail: warp::filters::path::Tail,  api_key: String| {
                let tail_str = tail.as_str().to_string();
                
                // println!("Forwarding to: {}", tail_str); // Debugging output
                // Forward the request including the path and query string
                proxy_request_handler::request_handler(tail_str, api_key, cache_clone.clone(), users_clone.clone())
            }
    });

    // Define the filtered proxy route
    let proxy_filtered_requests = warp::path("proxy")
        .and(warp::get())
        .and(warp::path::tail())
        .and(warp::query::raw())
        .and(warp::header::<String>("x-api-key"))
        .and_then({
            let cache_clone = cache.clone(); 
            let users_clone = users.clone(); 
            move |tail: warp::filters::path::Tail, query: String, api_key: String| {
                let full_path = if query.is_empty() {
                    tail.as_str().to_string()
                } else {
                    format!("{}?{}", tail.as_str(), query)
                };
                // println!("Forwarding to: {}", full_path);
                // Forward the request including the path and query string
                proxy_request_handler::request_handler(full_path, api_key, cache_clone.clone(), users_clone.clone())
            }
        });

    //WARNING: the order of the or clause does matter, because the filtered requests pass also the check of the std requests
    // the opposite doesn't hold. 
    let routes = signup.or(proxy_filtered_requests).or(proxy_std_requests);
    
    // Set up and start the Warp server
    let addr = (ADDRESS.parse::<std::net::IpAddr>().unwrap(), PORT);
    println!("Proxy listening on {}:{}", ADDRESS, PORT);
    warp::serve(routes).run(addr).await;

}



