use clap::{Arg, Command};


pub fn build_cli() -> Command {
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