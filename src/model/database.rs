use model::route::Route;
use model::stop::Stop;
use std::collections::HashMap;
use model::prediction::{RoutePrediction,
                        StopPrediction};


pub struct Database {
    routes: HashMap<String, Route>,
    stops: HashMap<String, Stop>
}

impl Database {
    pub fn new() -> Database {
        Database {
            routes: HashMap::new(),
            stops: HashMap::new()
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.entry(route.id.clone()).or_insert(route);
    }

    pub fn add_stop(&mut self, stop: Stop) {
        self.stops.entry(stop.id.clone()).or_insert(stop);
    }

    pub fn get_route(&self, id: &str) -> Option<&Route> {
        self.routes.get(id)
    }

    pub fn get_stop(&self, id: &str) -> Option<&Stop> {
        self.stops.get(id)
    }

    pub fn get_stops(&self) -> Vec<&Stop> {
        let mut stops: Vec<&Stop> = self.stops.iter().map(|(_, stop)| stop).collect();
        stops.sort_by(|a, b| a.name.cmp(&b.name));
        stops
    }

    // Return a vector of routes
    pub fn get_all(&self) -> Vec<&Route> {
        self.routes.iter().map(|(_, route)| route).collect()
    }

    pub fn update_route_arrivals(&mut self, route_id: String, prediction: RoutePrediction) {
        // Update route
        if let Some(route) = self.routes.get_mut(&route_id) {
            route.active = prediction.active;
            for stop in prediction.stops {
                route.update_arrivals(stop.id, stop.arrivals);
            }
        }
    }

    pub fn update_stop_arrivals(&mut self, stop_id: String, prediction: StopPrediction) {
        // Update stop
        if let Some(stop) = self.stops.get_mut(&stop_id) {
            stop.active = prediction.active;
            for route in prediction.routes {
                stop.update_arrivals(route.id, route.arrivals);
            }
        }
    }
}


