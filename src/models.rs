
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    count: i32,
    pages: i32,
    next: Option<String>,
    prev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    error: String
}

