use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, body};
use serde::Deserialize;
use crate::{proxy_signup, proxy_request_handler};
use serde_json::Value;


pub type Users = Arc<Mutex<HashMap<String, String>>>;
pub type Cache = Arc<Mutex<HashMap<String, Value>>>;

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
                proxy_signup::signup_handler(users_clone.clone(), signup_info.username) // Pass the extracted username to the handler
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
                proxy_request_handler::request_handler(endpoint, api_key, cache_clone.clone(), users_clone.clone()) // Clone for handler
            }
        });

    let routes = signup.or(proxy);
    
    println!("Proxy listening on port 3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}



