# Rust Actix Web Server - Deployed to Shuttle.rs

## About

A simple Rust Actix Web Server deployed to Shuttle.rs

The server has one endpoint, '/api/openmeteo', which accepts a JSON object with the following keys:

* start_date: String
* end_date: String
* city: String

The server then returns a JSON object with the historical weather data for that location, including a bunch of calculated stats (highest, lowest, mean)

## Usage

To run the server locally, clone the repo and run the following command:

```bash
cargo shuttle run
```

To deploy the server to Shuttle.rs, run the following command:

```bash

cargo shuttle deploy

```

