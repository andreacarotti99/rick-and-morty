use warp::{http::StatusCode, Reply, Rejection, reply};
use crate::proxy::Users;

pub async fn signup_handler(users: Users, username: String) -> Result<impl Reply, Rejection> {
    let mut users = users.lock().await; // Acquiring the lock on users

    if users.contains_key(&username) {
        let error_message = reply::json(&"Username already exists");
        let with_status = reply::with_status(error_message, StatusCode::CONFLICT);
        println!("username: {} already exists! signup failed", username);
        return Ok(with_status);
    }
    
    let api_key = format!("{}_{}", username, users.len() + 1);
    
    let proxy_api_key_response = format!("your api key is: {}\n", api_key);
    users.insert(username.clone(), api_key.clone());

    let json_reply_api_key = reply::json(&proxy_api_key_response);
    let with_status = reply::with_status(json_reply_api_key, StatusCode::OK);
    println!("new signup! username: {}, api-key: {}", username, api_key);
    Ok(with_status)
}

