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
        Commands::FetchAllCharacters => {
            handle_calls::handle_fetch_all_characters().await;
        },
        Commands::FetchSingleCharacter {id } => {
            handle_calls::handle_fetch_single_character(id).await;
        },
        Commands::FetchFilteredCharacters {name, status, species, character_type, gender } => {
            handle_calls::handle_fetch_filtered_characters(name, status, species, character_type, gender).await;
        },
        Commands::FetchCharactersList {ids } => {
            handle_calls::handle_fetch_characters_list(ids).await;
        },
    }
}


    



