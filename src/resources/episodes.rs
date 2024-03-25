use serde::{Deserialize, Serialize};
use crate::models::Info;
use chrono::{DateTime, Utc};

/// Represents a single episode with its details.
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

/// Response structure used for fetching episodes, including pagination information.
#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeResponse {
    info: Info,
    results: Vec<Episode>,
}

/// Type alias for a list of episodes, used in bulk episode responses.
pub type EpisodeListResponse = Vec<Episode>;

/// Filters for querying episodes, supporting partial matching on various fields.
#[derive(Default, Serialize, Deserialize)]
pub struct FilterEpisode {
    pub name: Option<String>,
    pub episode: Option<String>
}

