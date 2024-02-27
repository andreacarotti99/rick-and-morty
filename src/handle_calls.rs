use crate::api;
use crate::resources::characters::FilterCharacter;
use crate::resources::locations::FilterLocation;
use crate::resources::episodes::FilterEpisode;
use crate::cli::Cli;

pub struct Handler {
    pub base_url: String,
    pub api_key: String
}

impl Handler {

    pub fn new(cli: &Cli) -> Self {
        let base_url = match &cli.proxy {
            Some(proxy_url) => format!("http://{}/proxy/", proxy_url), // Use the provided proxy URL as the base
            None => "https://rickandmortyapi.com/api/".to_string(), // Default base URL
        };
        let api_key = match &cli.key {
            Some(api_key) => format!("{}", api_key),
            None => format!("")
        };

        Handler {base_url, api_key}
    }
    pub async fn handle_send_signup(&self, username: String) {
        match api::send_signup( &username).await {
            Ok(response) => println!("{}", response),
            Err(e) => eprintln!("Error signing up: {}", e),
        }

    }

    // characters
    pub async fn handle_fetch_all_characters(&self) {
        match api::fetch_all_characters(self).await {
            Ok(response) => println!("All Characters: {:#?}", response),
            Err(e) => eprintln!("Error fetching all characters: {}", e),
        }
    }

    pub async fn handle_fetch_single_character(&self, id_str: String) {
        if let Ok(id) = id_str.parse::<i32>() {
            match api::fetch_single_character(&self, id).await {
                Ok(response) => println!("Single Character: {:#?}", response),
                Err(e) => eprintln!("Error fetching character {}: {}", id, e),
            }
        } else {
            eprintln!("Invalid character ID: {}", id_str);
        }
    }

    pub async fn handle_fetch_filtered_characters(&self,
        name: Option<String>,
        status: Option<String>,
        species: Option<String>,
        character_type: Option<String>,
        gender: Option<String>,
        ) {
        let filter_character = FilterCharacter {
            name,
            status,
            species,
            character_type,
            gender,
            ..Default::default() // if we want to extend filter with other filters
        };

        match api::fetch_filtered_characters(&self, &filter_character).await {
            Ok(response) => println!("Filtered Characters: {:#?}", response),
            Err(e) => eprintln!("Error fetching filtered characters: {}", e),
        }
    }

    pub async fn handle_fetch_characters_list(&self, ids: String) {
        let id_list: Vec<i32> = ids.split(',')
            .filter_map(|id| id.trim().parse::<i32>().ok())
            .collect();

        match api::fetch_characters_list(&self, &id_list).await {
            Ok(response) => println!("Characters List: {:#?}", response),
            Err(e) => eprintln!("Error fetching characters list: {}", e),
        }
    }

    //locations

    pub async fn handle_fetch_all_locations(&self) {
        match api::fetch_all_locations(&self).await {
            Ok(response) => println!("All Characters: {:#?}", response),
            Err(e) => eprintln!("Error fetching all locations: {}", e),
        }
    }

    pub async fn handle_fetch_single_location(&self, id_str: String) {
        if let Ok(id) = id_str.parse::<i32>() {
            match api::fetch_single_location(&self, id).await {
                Ok(response) => println!("Single Location: {:#?}", response),
                Err(e) => eprintln!("Error fetching location {}: {}", id, e),
            }
        } else {
            eprintln!("Invalid location ID: {}", id_str);
        }
    }

    pub async fn handle_fetch_filtered_locations(&self,
        name: Option<String>,
        location_type: Option<String>,
        dimension: Option<String>,
        ) {
        let filter_location = FilterLocation {
            name,
            location_type,
            dimension,
            ..Default::default() // if we want to extend filter with other filters
        };

        match api::fetch_filtered_locations( &self, &filter_location).await {
            Ok(response) => println!("Filtered Locations: {:#?}", response),
            Err(e) => eprintln!("Error fetching filtered locations: {}", e),
        }
    }

    pub async fn handle_fetch_multiple_locations(&self, ids: String) {
        let id_list: Vec<i32> = ids.split(',')
            .filter_map(|id| id.trim().parse::<i32>().ok())
            .collect();

        match api::fetch_location_list( &self, &id_list).await {
            Ok(response) => println!("Location List: {:#?}", response),
            Err(e) => eprintln!("Error fetching location list: {}", e),
        }
    }



    //episodes

    pub async fn handle_fetch_all_episodes(&self) {
        match api::fetch_all_episodes(&self).await {
            Ok(response) => println!("All episodes: {:#?}", response),
            Err(e) => eprintln!("Error fetching all episodes: {}", e),
        }
    }

    pub async fn handle_fetch_single_episode(&self, id_str: String) {
        if let Ok(id) = id_str.parse::<i32>() {
            match api::fetch_single_episode(&self, id).await {
                Ok(response) => println!("Single Episode: {:#?}", response),
                Err(e) => eprintln!("Error fetching episode {}: {}", id, e),
            }
        } else {
            eprintln!("Invalid episode ID: {}", id_str);
        }
    }

    pub async fn handle_fetch_filtered_episodes(&self, 
        name: Option<String>,
        episode: Option<String>,
        ) {
        let filter_episode = FilterEpisode {
            name,
            episode,
            ..Default::default() // if we want to extend filter with other filters
        };

        match api::fetch_filtered_episode(&self, &filter_episode).await {
            Ok(response) => println!("Filtered Locations: {:#?}", response),
            Err(e) => eprintln!("Error fetching filtered locations: {}", e),
        }
    }

    pub async fn handle_fetch_multiple_episodes(&self, ids: String) {
        let id_list: Vec<i32> = ids.split(',')
            .filter_map(|id| id.trim().parse::<i32>().ok())
            .collect();

        match api::fetch_episode_list( &self,&id_list).await {
            Ok(response) => println!("Episode List: {:#?}", response),
            Err(e) => eprintln!("Error fetching episode list: {}", e),
        }
    }

}


