mod api;
mod models;
mod resources;
mod cli;
mod handle_calls;

use tokio;

use crate::cli::Commands;
use clap::Parser;
use tokio::main;


#[main]
async fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        // characters
        Commands::FetchAllCharacters => {
            handle_calls::handle_fetch_all_characters().await;
        },
        Commands::FetchSingleCharacter {id } => {
            handle_calls::handle_fetch_single_character(id).await;
        },
        Commands::FetchFilteredCharacters {name, status, species, character_type, gender } => {
            handle_calls::handle_fetch_filtered_characters(name, status, species, character_type, gender).await;
        },
        Commands::FetchMultipleCharacters {ids } => {
            handle_calls::handle_fetch_characters_list(ids).await;
        },

        // locations
        Commands::FetchAllLocations => {
            handle_calls::handle_fetch_all_locations().await;
        },
        Commands::FetchSingleLocation {id } => {
            handle_calls::handle_fetch_single_location(id).await;
        },
        Commands::FetchFilteredLocations {name, location_type, dimension }=> {
            handle_calls::handle_fetch_filtered_locations(name, location_type, dimension).await;
        },
        Commands::FetchMultipleLocations {ids} => {
            handle_calls::handle_fetch_multiple_locations(ids).await;
        },

        //episodes
        Commands::FetchAllEpisodes => {
            handle_calls::handle_fetch_all_episodes().await;
        },
        Commands::FetchSingleEpisode {id} => {
            handle_calls::handle_fetch_single_episode(id).await;
        },
        Commands::FetchFilteredEpisodes {name, episode} => {
            handle_calls::handle_fetch_filtered_episodes(name, episode).await;
        },
        Commands::FetchMultipleEpisodes {ids} => {
            handle_calls::handle_fetch_multiple_episodes(ids).await;
        },
    }
}


    



