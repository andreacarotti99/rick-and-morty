
use serde::{Deserialize, Serialize};

/// Represents pagination information for API responses.
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    count: i32,
    pages: i32,
    next: Option<String>,
    prev: Option<String>,
}

/// Represents an error response from the API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    error: String
}

/// A trait defining a method to convert filters into a query string.
pub trait Filter {
    fn to_query_string(&self) -> String;
}

//to avoid rewriting the same function for every filter
/// Generic implementation of `Filter` for all types that can be serialized.
/// This allows any serializable type to be used as a filter for API requests,
/// converting its fields into a query string format.
impl<T: Serialize> Filter for T {
    fn to_query_string(&self) -> String {
        serde_qs::to_string(self).unwrap_or_default()
    }
}


/// Represents an API key structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub api_key: String,
}
