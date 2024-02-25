use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = "Rick and Morty CLI", version = "1.0", author = "Andrea Carotti", about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, help = "Specify a proxy URL")]
    //default_value = http://localhost:3030/
    pub proxy: Option<String>,

    
    #[arg(long, help = "Specify an API key")]
    pub key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {

    //characters
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
    FetchMultipleCharacters {
        #[arg(help = "The IDs of the characters, separated by commas")]
        ids: String,
    },

    //locatons
    FetchAllLocations,
    FetchSingleLocation {
        #[arg(help = "The ID of the location")]
        id: String,
    },
    FetchFilteredLocations {
        #[arg(long, help = "Filter by location name")]
        name: Option<String>,
        
        #[arg(long, help = "Filter by location type")]
        location_type: Option<String>,
        
        #[arg(long, help = "Filter by location dimension")]
        dimension: Option<String>,
    },
    FetchMultipleLocations {
        #[arg(help = "The IDs of the locations, separated by commas")]
        ids: String,
    },

    //episodes
    FetchAllEpisodes,
    FetchSingleEpisode {
        #[arg(help = "The ID of the episode")]
        id: String,
    },
    FetchFilteredEpisodes {
        #[arg(long, help = "Filter by episode name")]
        name: Option<String>,
        
        #[arg(long, help = "Filter by episode code")]
        episode: Option<String>,
       
    },
    FetchMultipleEpisodes {
        #[arg(help = "The IDs of the episodes, separated by commas")]
        ids: String,
    },

    // proxy
    StartProxy,
    SignUp {
        #[arg(help = "The username to request the API key")]
        username: String,
    }
}
