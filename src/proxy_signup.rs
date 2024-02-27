use warp::{http::StatusCode, Reply, Rejection, reply};
use crate::proxy::Users;
use uuid::Uuid;

pub async fn signup_handler(users: Users, username: String) -> Result<impl Reply, Rejection> {
    let mut users = users.lock().await; // Acquiring the lock on users

    if users.values().any(|v| v == &username) {
        let error_message = warp::reply::json(&"Username already exists");
        let with_status = warp::reply::with_status(error_message, warp::http::StatusCode::CONFLICT);
        println!("Username: {} already exists! Signup failed.", username);
        return Ok(with_status);
    }
    
    let api_key = Uuid::new_v4().to_string();

    
    let proxy_api_key_response = format!("your api key is: {}", api_key);
    users.insert( api_key.clone(), username.clone());

    let json_reply_api_key = reply::json(&proxy_api_key_response);
    let with_status = reply::with_status(json_reply_api_key, StatusCode::OK);
    println!("New signup! username: {}, api-key: {}", username, api_key);
    Ok(with_status)
}

