use warp::{http::StatusCode, Reply, Rejection, reply};
use crate::proxy::Users;
use uuid::Uuid;

/// Handles the signup process, creating a new user with a unique API key.
///
/// This function checks if the username already exists in the shared `Users` structure. If not,
/// it generates a new API key for the user and inserts it into `Users`. It returns an API key
/// for successful signups or an error message if the username already exists.
///
/// # Arguments
/// * `users` - A shared `Users` structure to store and validate usernames and API keys.
/// * `username` - The username submitted for signup.
///
/// # Returns
/// A `Result` with a `warp::Reply` that contains the new API key for successful signups or
/// an error message for failed signups, and a `Rejection` if an unexpected error occurs.
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

