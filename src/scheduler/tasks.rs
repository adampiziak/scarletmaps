use std::sync::{Arc, RwLock};
use std::error::Error;

use model::{ Database };
use api::{ nextbus, builder };

pub fn update_route_config(database: Arc<RwLock<Database>>) -> Result<(), Box<Error>>{
    let config_json = nextbus::get_configuration()?;
    builder::parse_config(database, config_json);
    Ok(())
}
