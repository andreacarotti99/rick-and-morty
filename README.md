# RickAndMortyCLI

RickAndMortyCLI is a command-line interface application designed to consume the Rick and Morty API, allowing users to fetch data about characters, locations, and episodes from the show.

## Features

To run the program:
`cargo run -- <arguments>`
where arguments is substituted by the commands below.


- **Characters:**
  - `fetch-all-characters`: Fetch all characters.
  - `fetch-single-character <id>`: Fetch a single character by ID.
  - `fetch-filtered-characters [--name <name>] [--status <status>] [--species <species>] [--character-type <type>] [--gender <gender>]`: Fetch characters with optional filters.
  - `fetch-multiple-characters <id1,id2,...>`: Fetch multiple characters by their IDs.

- **Locations:**
  - `fetch-all-locations`: Fetch all locations.
  - `fetch-single-location <id>`: Fetch a single location by ID.
  - `fetch-filtered-locations [--name <name>] [--location-type <type>] [--dimension <dimension>]`: Fetch locations with optional filters.
   - `fetch-multiple-locations <id1,id2,...>`: Fetch multiple locations by their IDs.

- **Episodes:**
    - `fetch-all-episodes`: Fetch all episodes.
    - `fetch-single-episode <id>`: Fetch a single episode by ID.
    - `fetch-filtered-episodes [--name <name>] [--episode <episode_code>]`: Fetch episodes with optional filters.
    - `fetch-multiple-episodes <id1,id2,...>`: Fetch multiple episodes by their IDs.

