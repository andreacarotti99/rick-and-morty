use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = "Rick and Morty CLI", version = "1.0", author = "Andrea Carotti", about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    
    FetchAllCharacters,

    FetchSingleCharacter {
        #[arg(help = "The ID of the character")]
        id: String,
    },
   
    FetchFilteredCharacters {
        #[arg(long, help = "Filter by character name")]
        name: Option<String>,
        
        #[arg(long, help = "Filter by character status")]
        status: Option<String>,
        
        #[arg(long, help = "Filter by species")]
        species: Option<String>,
        
        #[arg(long, help = "Filter by type", name = "character_type")]
        character_type: Option<String>,
        
        #[arg(long, help = "Filter by gender")]
        gender: Option<String>,
    },
    
    FetchCharactersList {
        #[arg(help = "The IDs of the characters, separated by commas")]
        ids: String,
    },
}
