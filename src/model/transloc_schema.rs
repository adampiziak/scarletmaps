use juniper::Context;
use model::transloc::{TranslocDatabase, Route, Stop, Vehicle};
use api::lookup;

impl Context for TranslocDatabase {}

pub struct RoutePair<'a>(&'a Route, i32); // 2nd arg is parent stop id
pub struct StopPair<'a>(&'a Stop, i32); // 2nd arg is parent route id

graphql_object!(TranslocDatabase: TranslocDatabase as "Query" |&self| {
    description: "The root query of the Transloc graphql database"
        
    field route(&executor, id: i32 as "route id (required)") -> Option<RoutePair> {
        let route = executor.context().get_route(&id);
        match route {
            Some(r) => Some(RoutePair(r, 0)),
            None => None
        } 
    }

    field routes(&executor, active: Option<bool>) -> Vec<RoutePair> {
        let db = executor.context();
        let routes_iter = db.get_routes().into_iter();
        match active {
            Some(a) => routes_iter
                .filter(|r| db.route_status(&r) == a)
                .map(|r| RoutePair(r, 0))
                .collect(),
            None => routes_iter.map(|r| RoutePair(r, 0)).collect()
        }
    }

    field vehicles(&executor, id: i32 as "route id (required)")  -> Option<&Vec<Vehicle>> {
        let db = executor.context();
        db.get_route_vehicles(&id)
    }

    field stop(&executor, id: i32 as "stop id (required)") -> Option<StopPair> {
        let stop = executor.context().get_stop(&id);
        match stop {
            Some(r) => Some(StopPair(r, 0)),
            None => None
        } 
    }

    field stops(&executor, active: Option<bool>) -> Vec<StopPair> {
        let db = executor.context();
        let stops_iter = db.get_stops().into_iter();
        match active {
            Some(a) => stops_iter
                .filter(|s| db.stop_status(&s) == a)
                .map(|s| StopPair(s, 0))
                .collect(),
            None => stops_iter.map(|s| StopPair(s, 0)).collect()
        }
    }
});


graphql_object!(<'a> RoutePair<'a>: TranslocDatabase as "RoutePair" |&self| {
    field id() -> &i32 {
        &self.0.id
    }

    field name() -> &String {
        &self.0.name
    }

    field active(&executor) -> bool {
        let db = executor.context();
        let stops_iter = db.get_stops_by_ids(&self.0.served_stops).into_iter();
        let mut active = false;
        for stop in stops_iter {
            match db.arrivals.get(&(self.0.id, stop.id)) {
                Some(arrivals) => {
                    if arrivals.len() > 0 {
                        active = true;
                    }
                },
                None => ()
            }
        }
        return active
    }
    
    field stops(&executor, active: Option<bool>) -> Vec<StopPair> {
        let db = executor.context();
        let stops_iter = db.get_stops_by_ids(&self.0.served_stops).into_iter();
        match active {
            Some(a) => stops_iter
                .filter(|r| db.stop_status(&r) == a)
                .map(|r| StopPair(r, self.0.id))
                .collect(),
            None => stops_iter.map(|r| StopPair(r, self.0.id)).collect()
        }
    }

    field arrivals(&executor) -> Option<&Vec<f64>> {
        executor.context().arrivals.get(&(self.0.id, self.1))
    }

    field segments() -> &Vec<Vec<Vec<f64>>> {
        &self.0.segments
    }

    field areas() -> Vec<String> {
        lookup::get_route_areas(&self.0.served_stops)
    }
});

graphql_object!(Vehicle: TranslocDatabase as "Vehicle" |&self| {
    field id() -> &i32 {
        &self.id
    }

    field location() -> Vec<f64> {
        vec![self.location.1, self.location.0]
    }    
});

graphql_object!(<'a> StopPair<'a>: TranslocDatabase as "StopPair" |&self| {
    field id() -> &i32 {
        &self.0.id
    }

    field name() -> &String {
        &self.0.name
    }

    field location() -> Vec<f64> {
        vec![self.0.location.1, self.0.location.0]
    }

    field area() -> String {
        lookup::get_stop_area(&self.0.id)
    }

    field areas() -> &Vec<String> {
        &self.0.served_areas
    }

    field routes(&executor, active: Option<bool>) -> Vec<RoutePair> {
        let db = executor.context();
        let routes_iter = db.get_routes_by_ids(&self.0.served_routes).into_iter();
        match active {
            Some(a) => routes_iter
                .filter(|r| db.route_status(&r) == a)
                .map(|r| RoutePair(r, self.0.id))
                .collect(),
            None => routes_iter.map(|r| RoutePair(r, self.0.id)).collect()
        }
    }

    field arrivals(&executor) -> Option<&Vec<f64>> {
        executor.context().arrivals.get(&(self.1, self.0.id))
    }
});

graphql_object!(Route: TranslocDatabase |&self| {
    field id() -> &i32 {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field stops(&executor) -> Vec<&Stop> {
        executor.context().get_stops_by_ids(&self.served_stops)
    }
});


graphql_object!(Stop: TranslocDatabase |&self| {
    field id() -> &i32 {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field routes(&executor) -> Vec<&Route> {
        executor.context().get_routes_by_ids(&self.served_routes)
    }
});
