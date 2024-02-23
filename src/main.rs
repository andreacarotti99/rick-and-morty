mod api;
mod models;
mod resources;
mod cli;
mod handle_calls;

use crate::cli::{Cli, Commands};
use clap::Parser;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dispatch_command(cli.command).await;
}

async fn dispatch_command(command: Commands) {
    use Commands::*;
    
    match command {
        // Characters
        FetchAllCharacters => handle_calls::handle_fetch_all_characters().await,
        FetchSingleCharacter { id } => handle_calls::handle_fetch_single_character(id).await,
        FetchFilteredCharacters {
            name,
            status,
            species,
            character_type,
            gender,
        } => handle_calls::handle_fetch_filtered_characters(name, status, species, character_type, gender).await,
        FetchMultipleCharacters { ids } => handle_calls::handle_fetch_characters_list(ids).await,

        // Locations
        FetchAllLocations => handle_calls::handle_fetch_all_locations().await,
        FetchSingleLocation { id } => handle_calls::handle_fetch_single_location(id).await,
        FetchFilteredLocations {
            name,
            location_type,
            dimension,
        } => handle_calls::handle_fetch_filtered_locations(name, location_type, dimension).await,
        FetchMultipleLocations { ids } => handle_calls::handle_fetch_multiple_locations(ids).await,

        // Episodes
        FetchAllEpisodes => handle_calls::handle_fetch_all_episodes().await,
        FetchSingleEpisode { id } => handle_calls::handle_fetch_single_episode(id).await,
        FetchFilteredEpisodes { name, episode } => handle_calls::handle_fetch_filtered_episodes(name, episode).await,
        FetchMultipleEpisodes { ids } => handle_calls::handle_fetch_multiple_episodes(ids).await,
    }
}
