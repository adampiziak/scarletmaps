// Interface to Transloc Servers
// Rutgers University ID: 1323

use std::env;
use serde_json;
use std::error::Error;
use reqwest::Client;
use reqwest::header::{self, HeaderValue};
use std::collections::HashMap;
use polyline;

use model::transloc_api::{Routes, Route, Stops, Stop, ArrivalEstimates, VehicleData};

pub fn fetch_routes() -> Result<Vec<Route>, Box<Error>> {
    let client = get_client();
    let res = client
        .get("https://transloc-api-1-2.p.mashape.com/routes.json?agencies=1323&callback=call")
        .send()?
        .text()?;
    
    let transloc_routes: Routes = serde_json::from_str(&res)?;
    
    Ok(transloc_routes.data.routes)
}

pub fn fetch_arrival_estimates() -> Result<ArrivalEstimates, Box<Error>> {
    let client = get_client();
    let res = client
        .get("https://transloc-api-1-2.p.mashape.com/arrival-estimates.json?agencies=1323&callback=call")
        .send()?
        .text()?;

    let transloc_arrival_estimates: ArrivalEstimates = serde_json::from_str(&res)?;
    Ok(transloc_arrival_estimates)
}

pub fn fetch_stops() -> Result<Vec<Stop>, Box<Error>> {
    let client = get_client();
    let res = client
        .get("https://transloc-api-1-2.p.mashape.com/stops.json?agencies=1323&callback=call")
        .send()?
        .text()?;

    let transloc_stops: Stops = serde_json::from_str(&res)?;

    Ok(transloc_stops.data)
}

pub fn get_client() -> Client {
    let key = env::var("TRANSLOC_KEY").expect("NO TRANSLOC KEY");
    let mut headers = header::HeaderMap::new();
    headers.insert("X-Mashape-Key", HeaderValue::from_str(&key).unwrap());
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap();

    client
}

#[derive(Deserialize)]
struct Segments {
    data: HashMap<String, String>
}

// 4012616
pub fn fetch_segments(route_id: i32) -> Result<Vec<Vec<Vec<f64>>>, Box<Error>>{
    let client = get_client();
    let url = format!(
        "https://transloc-api-1-2.p.mashape.com/segments.json?agencies=1323&callback=call&routes={}",
        route_id
    );
    let res = client
        .get(&url)
        .send().unwrap()
        .text().unwrap();

    let m: Segments = serde_json::from_str(&res).unwrap();

    let mut segments = Vec::new();
    for (_key, value) in m.data {
        let polyline = polyline::decode_polyline(&value, 5)
            .unwrap()
            .into_iter()
            .map(|a| a.to_vec())
            .collect();
        segments.push(polyline);
    }

    Ok(segments)
}

pub fn fetch_vehicle_locations() -> Result<VehicleData, Box<Error>> {
    let client = get_client();
    let url = "https://transloc-api-1-2.p.rapidapi.com/vehicles.json?callback=call&agencies=1323";

    let res = client.get(url)
        .send().unwrap()
        .text().unwrap();

    let parsed_res: VehicleData = serde_json::from_str(&res).unwrap();
    
    Ok(parsed_res)
}
