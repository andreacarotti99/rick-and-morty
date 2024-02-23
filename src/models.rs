
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    count: i32,
    pages: i32,
    next: Option<String>,
    prev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    id: i32,
    name: String,
    status: String,
    species: String,
    #[serde(rename = "type")]
    character_type: String,
    gender: String,
    origin: Location,
    location: Location,
    image: String,
    episode: Vec<String>,
    url: String,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    url: String,
}

// Response for all characters and a single page
#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersResponse {
    info: Info,
    results: Vec<Character>,
}

// Response for a single character
//#[derive(Serialize, Deserialize, Debug)]
//pub struct SingleCharacterResponse(pub Character);

// Response for a list of characters (without pagination info)
pub type CharactersListResponse = Vec<Character>;

#[derive(Default, Serialize)]
pub struct Filter {
    pub name: Option<String>,
    pub status: Option<String>,
    pub species: Option<String>,
    #[serde(rename = "type")]
    pub character_type: Option<String>,
    pub gender: Option<String>,
}

/* 
impl Filter {
    pub fn to_query_string(&self) -> String {
        let mut queries = vec![];

        // we check over the fields of the filter object if a filter was set

        if let Some(ref name) = self.name {
            queries.push(format!("name={}", name));
        }
        if let Some(ref status) = self.status {
            queries.push(format!("status={}", status));
        }
        if let Some(ref species) = self.species {
            queries.push(format!("species={}", species));
        }
        if let Some(ref character_type) = self.character_type {
            queries.push(format!("character_type={}", character_type));
        }
        if let Some(ref gender) = self.gender {
            queries.push(format!("gender={}", gender));
        }
        // Add other filters similarly...

        if queries.is_empty() {
            String::new()
        } else {
            format!("?{}", queries.join("&"))
        }
    }
}
*/

impl Filter {
    pub fn to_query_string(&self) -> String {
        // Use serde_qs to serialize the Filter struct into a query string
        let query = serde_qs::to_string(self).unwrap_or_default();
        println!("{}", query);
        query
    }
}