use std::sync::{Arc, RwLock};

use model::{Database, Route};
use model::nextbus::{Config};

pub fn parse_config(database: Arc<RwLock<Database>>, config: Config) {
    let mut database = database.write().unwrap();
    for route in config.route.into_iter() {
        database.update_route(Route::new(route.tag, route.title));
    }
}

