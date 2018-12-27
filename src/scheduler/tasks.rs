use std::sync::{Arc, RwLock};
use std::error::Error;
use reqwest::Client;

use model::transloc_api::{Routes};
use model::database::{Database, SyncedDatabase};
use api::{ nextbus, transloc, builder };

// Nextbus
pub fn update_route_config(database: Arc<RwLock<Database>>) -> Result<(), Box<Error>>{
    let config = nextbus::get_configuration()?;
    builder::parse_config(database, config);
    Ok(())
}

pub fn update_route_predictions(database: Arc<RwLock<Database>>) {
    let mut query_str = String::new();
    
    {
        let db_read = database.read().unwrap();
        let routes = db_read.get_all();


        for route in &routes {
            for stop in &route.stops {
                let query = format!("&stops={}|{}", route.id, stop.id);
                query_str.push_str(&query);
            }
        }
    }
    
    let schedule = nextbus::get_predictions(query_str).unwrap();
    builder::parse_predictions(database, schedule);
}

// Transloc
pub fn update_routes_via_transloc(database: SyncedDatabase) {
    let routes = transloc::fetch_routes().unwrap();
    for route in routes.data.routes {
        println!("{}", route.long_name);
    }
}

pub fn update_stops_via_transloc(database: SyncedDatabase) {
    let stops = transloc::fetch_stops().unwrap();
    for stop in stops.data {
        println!("{}", stop.name);
    }
}
