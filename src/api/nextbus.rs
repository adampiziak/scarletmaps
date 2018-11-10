use std::error::Error;
use serde_json;
use reqwest;
use std::sync::{Arc, RwLock};

use model::Database;
use model::nextbus::{ Config, Schedule };


static CONFIG_URL: &str =
    "http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers";

static PREDICTIONS_URL: &str =
    "http://webservices.nextbus.com/service/publicJSONFeed?a=rutgers&command=predictionsForMultiStops";

pub fn get_configuration() -> Result<Config, Box<Error>> {
    let config_str = reqwest::get(CONFIG_URL)?.text()?;
    
    let deserialized: Config = serde_json::from_str(&config_str).unwrap();
    Ok(deserialized)
}

pub fn get_predictions(query: String) -> Result<Schedule, Box<Error>> {
    let url = format!("{}{}", PREDICTIONS_URL, query);
    let predictions_str = reqwest::get(&url)?.text()?;

    let deserialized: Schedule = serde_json::from_str(&predictions_str).unwrap();
    Ok(deserialized)
}
