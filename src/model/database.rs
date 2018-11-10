use model::route::Route;
use std::collections::HashMap;

pub struct Database {
    routes: HashMap<String, Route>
}

impl Database {
    pub fn new() -> Database {
        Database { routes: HashMap::new() }
    }

    pub fn update_route(&mut self, route: Route) {
        self.routes.entry(route.id.clone()).or_insert(route);
    }

    pub fn get_route(&self, id: &str) -> Option<&Route> {
        self.routes.get(id)
    }

    // Return a vector of routes
    pub fn get_all(&self) -> Vec<&Route> {
        self.routes.iter().map(|(_, route)| route).collect()
    }

    pub fn update_route_arrivals(&mut self, route_id: String, stop_id: String, times: Vec<f64>) {
        if let Some(route) = self.routes.get_mut(&route_id) {
            route.update_arrivals(stop_id, times);
        }
    }
}


