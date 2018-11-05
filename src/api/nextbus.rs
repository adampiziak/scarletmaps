use std::error::Error;
use serde_json;
use reqwest;

use model::nextbus::{ Config };

pub fn get_configuration() -> Result<Config, Box<Error>> {
    let config_str = reqwest::get("https://api.ruroutes.com/config")?.text()?;
    
    let deserialized: Config = serde_json::from_str(&config_str).unwrap();
    Ok(deserialized)
}
