use std::sync::{Arc, RwLock};
use std::error::Error;

use model::{ Database };
use api::{ nextbus, builder };

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
