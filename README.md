# RickAndMortyCLI

RickAndMortyCLI is a command-line interface application designed to consume the Rick and Morty API, allowing users to fetch data about characters, locations, and episodes from the show.

## Features

- **Characters:**
  - `fetch-all-characters`: Fetch all characters.
  - `fetch-single-character <id>`: Fetch a single character by ID.
  - `fetch-filtered-character [--name <name>] [--status <status>] [--species <species>] [--type <type>] [--gender <gender>]`: Fetch characters with optional filters.
  - `fetch-multiple-characters <id1,id2,...>`: Fetch multiple characters by their IDs.

- **Locations:**
  - `fetch-all-locations`: Fetch all locations.
  - `fetch-filtered-location [--name <name>] [--type <type>] [--dimension <dimension>]`: Fetch locations with optional filters.

- **Episodes:**
  - `fetch-all-episodes`: Fetch all episodes.
  - `fetch-filtered-episode [--name <name>] [--episode <episode_code>]`: Fetch episodes with optional filters.

