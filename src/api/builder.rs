// This module takes a response from Transloc or NextBus and converts it into
// this server's model

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use chrono::{DateTime};

use api::lookup;
use model::nextbus::{NextBusDatabase, Route, RouteStop, Stop, StopRoute};
use model::nextbus_api::{Config, Schedule};
use model::transloc::{TranslocDatabase, self};
use model::transloc_api;

use model::prediction::{RoutePrediction,
                        StopPrediction,
                        StopRoutePrediction,
                        RouteStopPrediction};

// Transloc
pub fn update_route_list(database: Arc<RwLock<TranslocDatabase>>, routes: Vec<transloc_api::Route>) {
    let mut db = database.write().unwrap();
    for route in routes {
        db.routes.entry(route.route_id.clone())
            .or_insert(transloc::Route::new(route.route_id, route.long_name, route.stops));
    }
}

pub fn update_stop_list(database: Arc<RwLock<TranslocDatabase>>, stops: Vec<transloc_api::Stop>) {
    let mut db = database.write().unwrap();
    for stop in stops {
        db.stops.entry(stop.stop_id.clone())
            .or_insert(transloc::Stop::new(stop.stop_id, stop.name, stop.routes));
    }
}

pub fn update_arrival_estimates(database: Arc<RwLock<TranslocDatabase>>,
                                estimates: transloc_api::ArrivalEstimates) {
    let mut db = database.write().unwrap();
    db.arrivals.clear();
    for stop in estimates.data {
        for route in stop.arrivals {
            let time = DateTime::parse_from_rfc3339(&route.arrival_at).unwrap().timestamp_millis() as f64;
            let mut times = db.arrivals.entry((route.route_id, stop.stop_id)).or_insert(Vec::new());
            times.push(time);
        }
    }
    
}


// Nextbus
pub fn parse_config(database: Arc<RwLock<NextBusDatabase>>, config: Config) {
    let mut routes = HashMap::new();
    let mut stops = HashMap::new();
    
    for r in config.route.into_iter() {
        let route = routes.entry(r.tag.clone()).or_insert(Route::new(r.tag.clone(), r.title.clone()));
        for s in r.stop.into_iter() {
            let stop_campus = lookup::stop_campus(&s.tag);
            let stop = stops.entry(s.tag.clone())
                .or_insert(Stop::new(s.tag.clone(), s.title.clone(), stop_campus.clone()));
            stop.routes.push(StopRoute::new(r.tag.clone(), r.title.clone()));
            route.stops.push(RouteStop::new(s.tag, s.title, stop_campus));
        }
    }

    let mut database = database.write().unwrap();
    for (_, route) in routes {
        database.add_route(route);
    }

    for (_, stop) in stops {
        database.add_stop(stop);
    }
}


pub fn parse_predictions(database: Arc<RwLock<NextBusDatabase>>, schedule: Schedule) {
    let mut routes = HashMap::new();
    let mut stops  = HashMap::new();

    for nextbus_prediction in schedule.predictions.unwrap().into_iter() {
        let route_id = nextbus_prediction.route_tag;
        let stop_id = nextbus_prediction.stop_tag;

        let route_pred = RoutePrediction::new(route_id.clone());
        let stop_pred = StopPrediction::new(stop_id.clone());
        
        let route = routes.entry(route_id.clone()).or_insert(route_pred);
        let stop  = stops.entry(stop_id.clone()).or_insert(stop_pred);
        
        if let Some(direction) = nextbus_prediction.direction {
            let arrivals: Vec<f64> = direction.prediction.iter().map(|x| x.epoch_time).collect();
            stop.active = true;
            route.active = true;
            stop.routes.push(StopRoutePrediction { id: route_id, arrivals: arrivals.clone() });
            route.stops.push(RouteStopPrediction { id: stop_id, arrivals });
        }
    }

    let mut database = database.write().unwrap();
    for (id, route) in routes {
        database.update_route_arrivals(id, route);
    }

    for (id, stop) in stops {
        database.update_stop_arrivals(id, stop);
    }


}
