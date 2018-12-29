use juniper::Context;
use model::transloc::{TranslocDatabase, Route, Stop};

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

    field routes(&executor) -> Vec<RoutePair> {
        let routes = executor.context().get_routes();
        routes.into_iter().map(|r| RoutePair(r, 0)).collect()
    }

    field stop(&executor, id: i32 as "stop id (required)") -> Option<&Stop> {
        executor.context().get_stop(&id)
    }

    field stops(&executor) -> Vec<&Stop> {
        executor.context().get_stops()
    }
});


graphql_object!(<'a> RoutePair<'a>: TranslocDatabase as "RoutePair" |&self| {
    field id() -> &i32 {
        &self.0.id
    }

    field name() -> &String {
        &self.0.name
    }

    field stops(&executor) -> Vec<StopPair> {
        let stops = executor.context().get_stops_by_ids(&self.0.served_stops);
        stops.into_iter().map(|s| StopPair(s, self.0.id)).collect()
    }

    field arrivals(&executor) -> Option<&Vec<f64>> {
        executor.context().arrivals.get(&(self.0.id, self.1))
    }
});

graphql_object!(<'a> StopPair<'a>: TranslocDatabase as "StopPair" |&self| {
    field id() -> &i32 {
        &self.0.id
    }

    field name() -> &String {
        &self.0.name
    }

    field routes(&executor) -> Vec<RoutePair> {
        let routes = executor.context().get_routes_by_ids(&self.0.served_routes);
        routes.into_iter().map(|r| RoutePair(r, self.0.id)).collect()
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
