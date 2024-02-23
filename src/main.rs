mod api;
mod models;
mod resources;
mod cli;


use tokio;
use crate::api::*;
use crate::resources::characters::Filter;
use tokio::main;



#[main]
async fn main() {
    let matches = cli::build_cli().get_matches();

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
        // still need to handle the case when the response is empty: "error" : "There is nothing here", now returns an error
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


