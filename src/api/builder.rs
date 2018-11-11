use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use api::lookup;
use model::{Database, Route, RouteStop, Stop, StopRoute};
use model::nextbus::{Config, Schedule};
use model::prediction::{RoutePrediction,
                        StopPrediction,
                        StopRoutePrediction,
                        RouteStopPrediction};

pub fn parse_config(database: Arc<RwLock<Database>>, config: Config) {
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


pub fn parse_predictions(database: Arc<RwLock<Database>>, schedule: Schedule) {
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
