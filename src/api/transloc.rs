// Rutgers University ID: 1323
use std::fs;
use serde_json;
use std::error::Error;
use reqwest::Client;
use reqwest::header::{self, HeaderValue};

use model::transloc_api::{Routes, Stops};

pub fn fetch_routes() -> Result<Routes, Box<Error>> {
    let client = get_client();
    let res = client
        .get("https://transloc-api-1-2.p.mashape.com/routes.json?agencies=1323&callback=call")
        .send()?
        .text()?;
    
    let transloc_routes: Routes = serde_json::from_str(&res)?;
    
    Ok(transloc_routes)
}

pub fn fetch_stops() -> Result<Stops, Box<Error>> {
    let client = get_client();
    let res = client
        .get("https://transloc-api-1-2.p.mashape.com/stops.json?agencies=1323&callback=call")
        .send()?
        .text()?;

    let transloc_stops: Stops = serde_json::from_str(&res)?;

    Ok(transloc_stops)
}

pub fn get_client() -> Client {
    let key = fs::read_to_string("TRANSLOC_KEY").expect("Transloc key file not found");
    let mut headers = header::HeaderMap::new();
    headers.insert("X-Mashape-Key", HeaderValue::from_str(&key).unwrap());
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap();


    client
}
