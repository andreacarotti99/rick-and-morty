
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

pub trait Filter {
    fn to_query_string(&self) -> String;
}

//to avoid rewriting the same function for every filter
impl<T: Serialize> Filter for T {
    fn to_query_string(&self) -> String {
        serde_qs::to_string(self).unwrap_or_default()
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub api_key: String,
}
