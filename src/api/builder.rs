use std::sync::{Arc, RwLock};

use model::{Database, Route, RouteStop};
use model::nextbus::{Config, Schedule};

pub fn parse_config(database: Arc<RwLock<Database>>, config: Config) {
    println!("Acquiring write lock <2> for database...");
    let mut database = database.write().unwrap();
    println!("Lock <2> acquired");
    for route in config.route.into_iter() {
        let mut stops = Vec::new();
        for stop in route.stop.into_iter() {
            stops.push(RouteStop::new(stop.tag, stop.title));
        }
        
        database.update_route(Route::new(route.tag, route.title, stops));
    }
    println!("Lock <2> released");
}

pub fn parse_predictions(database: Arc<RwLock<Database>>, schedule: Schedule) {
    println!("Acquiring write lock for schedule update");
    let mut database = database.write().unwrap();
    println!("Lock for schedule acquired");
    for prediction in schedule.predictions.unwrap().into_iter() {
        let route_id = prediction.route_tag;
        let stop_id = prediction.stop_tag;
        
        if let Some(d) = prediction.direction {
            let times: Vec<f64> = d.prediction.iter().map(|x| x.epochTime).collect();
            database.update_route_arrivals(route_id, stop_id, times);
        }
    }
    println!("Lock for schedule released");
}
