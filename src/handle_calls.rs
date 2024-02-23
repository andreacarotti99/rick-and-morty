use crate::api;
use crate::resources::characters::FilterCharacter;
use crate::resources::locations::FilterLocation;

// characters
pub async fn handle_fetch_all_characters() {
    match api::fetch_all_characters().await {
        Ok(response) => println!("All Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching all characters: {}", e),
    }
}

pub async fn handle_fetch_single_character(id_str: String) {
    if let Ok(id) = id_str.parse::<i32>() {
        match api::fetch_single_character(id).await {
            Ok(response) => println!("Single Character: {:#?}", response),
            Err(e) => eprintln!("Error fetching character {}: {}", id, e),
        }
    } else {
        eprintln!("Invalid character ID: {}", id_str);
    }
}

pub async fn handle_fetch_filtered_characters(
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

    match api::fetch_filtered_characters(&filter_character).await {
        Ok(response) => println!("Filtered Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching filtered characters: {}", e),
    }
}

pub async fn handle_fetch_characters_list(ids: String) {
    let id_list: Vec<i32> = ids.split(',')
        .filter_map(|id| id.trim().parse::<i32>().ok())
        .collect();

    match api::fetch_characters_list(&id_list).await {
        Ok(response) => println!("Characters List: {:#?}", response),
        Err(e) => eprintln!("Error fetching characters list: {}", e),
    }
}

//locations

pub async fn handle_fetch_all_locations() {
    match api::fetch_all_locations().await {
        Ok(response) => println!("All Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching all locations: {}", e),
    }
}

pub async fn handle_fetch_single_location(id_str: String) {
    if let Ok(id) = id_str.parse::<i32>() {
        match api::fetch_single_location(id).await {
            Ok(response) => println!("Single Location: {:#?}", response),
            Err(e) => eprintln!("Error fetching location {}: {}", id, e),
        }
    } else {
        eprintln!("Invalid location ID: {}", id_str);
    }
}

pub async fn handle_fetch_filtered_locations(
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

    match api::fetch_filtered_locations(&filter_location).await {
        Ok(response) => println!("Filtered Locations: {:#?}", response),
        Err(e) => eprintln!("Error fetching filtered locations: {}", e),
    }
}

pub async fn handle_fetch_multiple_locations(ids: String) {
    let id_list: Vec<i32> = ids.split(',')
        .filter_map(|id| id.trim().parse::<i32>().ok())
        .collect();

    match api::fetch_location_list(&id_list).await {
        Ok(response) => println!("Location List: {:#?}", response),
        Err(e) => eprintln!("Error fetching location list: {}", e),
    }
}


