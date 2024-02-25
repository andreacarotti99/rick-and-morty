mod api;
mod models;
mod resources;
mod cli;
mod handle_calls;
mod proxy;
mod proxy_signup;
mod proxy_request_handler;

use crate::cli::{Cli, Commands};
use clap::Parser;
use tokio;
use handle_calls::Handler;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    dispatch_command(cli).await;
}

async fn dispatch_command(cli: Cli) {
    use Commands::*;
    
    
    let handler = Handler::new(&cli);
    
    match cli.command {
        // Characters
        FetchAllCharacters => handler.handle_fetch_all_characters().await,
        FetchSingleCharacter { id } => handler.handle_fetch_single_character(id).await,
        FetchFilteredCharacters {
            name,
            status,
            species,
            character_type,
            gender,
        } => handler.handle_fetch_filtered_characters(name, status, species, character_type, gender).await,
        FetchMultipleCharacters { ids } => handler.handle_fetch_characters_list(ids).await,

        // Locations
        FetchAllLocations => handler.handle_fetch_all_locations().await,
        FetchSingleLocation { id } => handler.handle_fetch_single_location(id).await,
        FetchFilteredLocations {
            name,
            location_type,
            dimension,
        } => handler.handle_fetch_filtered_locations(name, location_type, dimension).await,
        FetchMultipleLocations { ids } => handler.handle_fetch_multiple_locations(ids).await,

        // Episodes
        FetchAllEpisodes => handler.handle_fetch_all_episodes().await,
        FetchSingleEpisode { id } => handler.handle_fetch_single_episode(id).await,
        FetchFilteredEpisodes { name, episode } => handler.handle_fetch_filtered_episodes(name, episode).await,
        FetchMultipleEpisodes { ids } => handler.handle_fetch_multiple_episodes(ids).await,
    
        // spin-up Proxy
        StartProxy => proxy::start_proxy().await,
        SignUp {username} => handler.handle_send_signup(username).await
    }
}
