mod api;
mod models;

use tokio;
use clap::{Arg, Command};
// use tokio::runtime::Runtime; // Import Runtime for blocking on async code
use crate::api::*;
use crate::models::Filter;
use tokio::main;



#[main]
async fn main() {
    let matches = build_cli().get_matches();

    //let rt = Runtime::new().unwrap(); // Create a new Tokio runtime

    match matches.subcommand() {
        Some(("fetch_all_characters", _)) => {
            handle_fetch_all_characters().await;
        },
        Some(("fetch_single_character", sub_matches)) => {
            handle_fetch_single_character(sub_matches).await;
        },
        Some(("fetch_filtered_characters", sub_matches)) => {
            handle_fetch_filtered_characters(sub_matches).await;
        },
        Some(("fetch_characters_list", sub_matches)) => {
            handle_fetch_characters_list(sub_matches).await;
        },


        // Other subcommands...
        _ => {} // Handle the case where no subcommand was used or matches a defined command
    }
}

    
async fn handle_fetch_all_characters() {
    match fetch_all_characters().await {
        Ok(response) => println!("All Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching all characters: {}", e),
    }
}

async fn handle_fetch_single_character(matches: &clap::ArgMatches) {
    if let Some(id_str) = matches.get_one::<String>("id") {
        if let Ok(id) = id_str.parse::<i32>() {
            match fetch_single_character(id).await {
                Ok(response) => println!("Single Character: {:#?}", response),
                Err(e) => eprintln!("Error fetching character {}: {}", id, e),
            }
        } else {
            eprintln!("Invalid character ID: {}", id_str);
        }
    }
}

async fn handle_fetch_filtered_characters(matches: &clap::ArgMatches) {
    let filter = Filter {
        name: matches.get_one::<String>("name").cloned(),
        status: matches.get_one::<String>("status").cloned(),
        species: matches.get_one::<String>("species").cloned(),
        character_type: matches.get_one::<String>("character_type").cloned(),
        gender: matches.get_one::<String>("gender").cloned(),
        ..Default::default()
    };
    

    match fetch_filtered_characters(&filter).await {
        Ok(response) => println!("Filtered Characters: {:#?}", response),
        Err(e) => eprintln!("Error fetching filtered characters: {}", e),
        }
    }

async fn handle_fetch_characters_list(matches: &clap::ArgMatches) {
    let id_list: Vec<i32> = matches.get_many::<String>("ids")
        .unwrap_or_default()
        .filter_map(|id| id.parse().ok())
        .collect();

    match fetch_characters_list(&id_list).await {
        Ok(response) => println!("Characters List: {:#?}", response),
        Err(e) => eprintln!("Error fetching characters list: {}", e),
    }
}


fn build_cli() -> Command {
    Command::new("Rick and Morty CLI")
        .version("1.0")
        .author("Andrea Carotti")
        .about("Consume the Rick and Morty API")

        .subcommand(Command::new("fetch_all_characters")
            .about("Fetches all characters"))
        
        .subcommand(Command::new("fetch_single_character")
            .about("Fetches a single character by ID")
            .arg(Arg::new("id")
                .help("The ID of the character")
                .required(true)
                .index(1)))
        
        .subcommand(Command::new("fetch_filtered_characters")
            .about("Fetches characters based on filters")
            .arg(Arg::new("name")
                .help("Filter by character name")
                .long("name"))
            .arg(Arg::new("status")
                .help("Filter by character status")
                .long("status"))
            .arg(Arg::new("species")
                .help("Filter by species")
                .long("species"))
            .arg(Arg::new("character_type")
                .help("Filter by type")
                .long("character_type"))
            .arg(Arg::new("gender")
                .help("Filter by gender")
                .long("gender")))
        .subcommand(Command::new("fetch_characters_list")
            .about("Fetches a list of characters by IDs")
            .arg(Arg::new("ids")
                .help("The IDs of the characters, separated by commas")
                .required(true)))
}

/* 
#[tokio::main]
async fn _main() {


    let filters = Filter {
        name: Some("rick".to_string()),
        status: Some("dead".to_string()),
        // Initialize other filters as needed...
        ..Default::default()
    };

    match fetch_filtered_characters(&filters).await {
        Ok(response) => println!("Filtered Characters: {:#?}", response),
        Err(e) => eprintln!("Error: {}", e),
    };

    match fetch_all_characters().await {
        Ok(response) => println!("ALL Characters: {:#?}", response),
        Err(e) => eprintln!("Error: {}", e),
    };

    match fetch_single_character(20).await {
        Ok(response) => println!("Single Character: {:#?}", response),
        Err(e) => eprintln!("Error: {}", e),
    };

    let character_ids = vec![1, 183]; // Ex
    match fetch_characters_list(&character_ids).await {
        Ok(response) => println!("List of Characters: {:#?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
    
}
*/