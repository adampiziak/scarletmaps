use std::collections::HashMap;

pub struct TranslocDatabase {
    pub routes: HashMap<i32, Route>,
    pub stops: HashMap<i32, Stop>,
    pub arrivals: HashMap<(i32, i32), Vec<f64>>, // Hashmap with key (routeid, stop_id) and times
    pub vehicles: HashMap<i32, Vec<Vehicle>>
}

impl TranslocDatabase {
    pub fn new() -> TranslocDatabase {
        TranslocDatabase {
            routes: HashMap::new(),
            stops: HashMap::new(),
            arrivals: HashMap::new(),
            vehicles: HashMap::new()
        }
    }

    pub fn get_route(&self, id: &i32) -> Option<&Route> {
        self.routes.get(id)
    }

    pub fn get_routes(&self) -> Vec<&Route> {
        let mut routes: Vec<&Route> = self.routes.iter().map(|(_, route)| route).collect();
        routes.sort_by_key(|&s| &s.name);
        routes
    }

    pub fn get_stop(&self, id: &i32) -> Option<&Stop> {
        self.stops.get(id)
    }

    pub fn get_route_vehicles(&self, id: &i32) -> Option<&Vec<Vehicle>> {
        self.vehicles.get(id)
    }


    pub fn get_stops(&self) -> Vec<&Stop> {
        let mut stops: Vec<&Stop> = self.stops.iter().map(|(_, stop)| stop).collect();
        stops.sort_by_key(|&s| &s.name);
        stops
            
    }

    pub fn get_stops_by_ids(&self, ids: &Vec<i32>) -> Vec<&Stop> {
        let mut stops = Vec::new();
        for id in ids {
            let stop = self.stops.get(&id).unwrap();
            stops.push(stop);
        }
        stops
    }

    pub fn get_routes_by_ids(&self, ids: &Vec<i32>) -> Vec<&Route> {
        let mut routes = Vec::new();
        for id in ids {
            let route = self.routes.get(&id).unwrap();
            routes.push(route);
        }
        routes
    }

    pub fn route_status(&self, route: &Route) -> bool {
        for &stop in &route.served_stops {
                match self.arrivals.get(&(route.id, stop)) {
                    Some(times) => return times.len() > 0,
                    None => continue
                };
        }

        false
    }

    pub fn stop_status(&self, stop: &Stop) -> bool {
        for &route_id in &stop.served_routes {
                match self.arrivals.get(&(route_id, stop.id)) {
                    Some(times) => return times.len() > 0,
                    None => continue
                };
        }

        false
    }
}

pub struct Route {
    pub id: i32,
    pub name: String,
    pub area_ids: Vec<i32>,
    pub served_stops: Vec<i32>,     // IDs of stops served
    pub segments: Vec<Vec<Vec<f64>>>
}

pub struct Stop {
    pub id: i32,
    pub name: String,
    pub location: (f64, f64),
    pub area_id: i32,
    pub served_routes: Vec<i32>,   // IDs of routes served
}

pub struct Vehicle {
    pub id: i32,
    pub location: (f64, f64)
}

impl Route {
    pub fn new(id: i32, name: String, stop_str_ids: Vec<String>) -> Route {
        let stop_ids: Vec<i32> = stop_str_ids.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

        Route { 
            id, 
            name,
            area_ids: Vec::new(),
            served_stops: stop_ids,
            segments: Vec::new()
        }
    }

}

impl Stop {
    pub fn new(id: i32, name: String, route_str_ids: Vec<String>, location: (f64, f64)) -> Stop {
        let route_ids = route_str_ids.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        Stop {
            id,
            name,
            location,
            area_id: 0,
            served_routes: route_ids,
        }
    }
}

impl Vehicle {
    pub fn new(id: i32, location: (f64, f64)) -> Vehicle {
        Vehicle {
            id,
            location
        }
    }
}
