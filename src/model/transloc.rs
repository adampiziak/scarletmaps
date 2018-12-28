use std::collections::HashMap;
use model::transloc_api;

pub struct TranslocDatabase {
    pub routes: HashMap<i32, Route>,
    pub stops: HashMap<i32, Stop>
}

impl TranslocDatabase {
    pub fn new() -> TranslocDatabase {
        TranslocDatabase {
            routes: HashMap::new(),
            stops: HashMap::new()
        }
    }

    pub fn get_route(&self, id: &i32) -> Option<&Route> {
        self.routes.get(id)
    }

    pub fn get_routes(&self) -> Vec<&Route> {
        self.routes.iter().map(|(_, route)| route).collect()
    }

    pub fn get_stop(&self, id: &i32) -> Option<&Stop> {
        self.stops.get(id)
    }

    pub fn get_stops(&self) -> Vec<&Stop> {
        self.stops.iter().map(|(_, stop)| stop).collect()
    }
}

pub struct Route {
    pub id: i32,
    pub name: String,
    pub area_ids: Vec<i32>,
    pub stops: Vec<Stop>,
    pub arrivals: Vec<f64>
}

pub struct Stop {
    pub id: i32,
    pub name: String,
    pub area_id: i32,
    pub routes: Vec<Route>,
    pub arrivals: Vec<f64>
}

impl Route {
    pub fn new(id: i32, name: String) -> Route {
        Route { 
            id, 
            name,
            area_ids: Vec::new(),
            stops: Vec::new(),
            arrivals: Vec::new()
        }
    }
}

impl Stop {
    pub fn new(id: i32, name: String) -> Stop {
        Stop {
            id,
            name,
            area_id: 0,
            routes: Vec::new(),
            arrivals: Vec::new()
        }
    }
}
