use serde::{Deserialize, Serialize};
use crate::models::Info;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub id: i32,
    pub name: String,
    pub air_date: String, 
    pub episode: String, 
    pub characters: Vec<String>, 
    pub url: String,
    pub created: DateTime<Utc>, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeResponse {
    info: Info,
    results: Vec<Episode>,
}


pub type EpisodeListResponse = Vec<Episode>;

#[derive(Default, Serialize, Deserialize)]
pub struct FilterEpisode {
    pub name: Option<String>,
    pub episode: Option<String>
}

