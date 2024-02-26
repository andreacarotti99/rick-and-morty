# RickAndMortyCLI

RickAndMortyCLI is a command-line interface application designed to consume the Rick and Morty API (https://rickandmortyapi.com), allowing users to fetch data about characters, locations, and episodes from the show.

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

## Extra Feature: Proxy

It is possible to set up a proxy running the command `cargo run -- start-proxy`.
Users can sign-up to the proxy using the command `cargo run -- sign-up <username>`. If no user with that username is already present in the temporary hashmap of the proxy, the user is provided with an api-key.

The user can retrieve cached data from the proxy using the command:
```
cargo run -- --proxy <proxy-address> --key <api-key> <arguments>
```
where `<arguments>` is one of the above commands to fetch data from the api.
If the proxy have not yet cached any data, the request will be forwarded to the server, otherwise will be served to the user. 

Currently, the data of the proxy is temporary. So users will have to sign-up again if the proxy is stopped. The same stands for the cached data. Once the proxy is turned off, all the cached data is deleted. Currently the proxy is hard-coded at the address `127.0.0.1` port `3030`.

Example:
- Open terminal (1) and run `cargo run -- start-proxy`
- Open terminal (2) and run `cargo run -- sign-up andrea`. Let's assume the API key is `andrea_1`
- Run in terminal (2) `cargo run -- --proxy 127.0.0.1:3030 --key andrea_1 fetch-all-characters`. A list of character from the server is provided
- Run again on terminal (2) `cargo run -- --proxy 127.0.0.1:3030 --key andrea_1 fetch-all-characters`. Now the results will be fetched from the cache of the proxy.


