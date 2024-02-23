use crate::api::*;
use crate::resources::characters::Filter;

pub async fn handle_fetch_all_characters() {
    match fetch_all_characters().await {
        Ok(response) => println!("All Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching all characters: {}", e),
    }
}

pub async fn handle_fetch_single_character(id_str: String) {
    if let Ok(id) = id_str.parse::<i32>() {
        match fetch_single_character(id).await {
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
    let filter = Filter {
        name,
        status,
        species,
        character_type,
        gender,
        ..Default::default() // if we want to extend filter with other filters
    };

    match fetch_filtered_characters(&filter).await {
        Ok(response) => println!("Filtered Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching filtered characters: {}", e),
    }
}

pub async fn handle_fetch_characters_list(ids: String) {
    let id_list: Vec<i32> = ids.split(',')
        .filter_map(|id| id.trim().parse::<i32>().ok())
        .collect();

    match fetch_characters_list(&id_list).await {
        Ok(response) => println!("Characters List: {:#?}", response),
        Err(e) => eprintln!("Error fetching characters list: {}", e),
    }
}