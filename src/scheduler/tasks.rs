// Tasks run by the scheduler

use std::sync::{Arc, RwLock};
use std::error::Error;
use api::{ nextbus, transloc, builder };
use model::nextbus::{NextBusDatabase};
use model::transloc::{TranslocDatabase};


// Nextbus
pub fn update_route_config(database: Arc<RwLock<NextBusDatabase>>) -> Result<(), Box<Error>>{
    let config = nextbus::get_configuration()?;
    builder::parse_config(database, config);
    Ok(())
}

pub fn update_route_predictions(database: Arc<RwLock<NextBusDatabase>>) {
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
pub fn update_routes_via_transloc(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let routes = transloc::fetch_routes().unwrap();
    builder::update_route_list(transloc_db, routes);
}

pub fn update_stops_via_transloc(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let stops = transloc::fetch_stops().unwrap();
    builder::update_stop_list(transloc_db, stops);
}
