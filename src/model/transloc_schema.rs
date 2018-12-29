use juniper::Context;
use model::transloc::{TranslocDatabase, Route, Stop};

impl Context for TranslocDatabase {}

graphql_object!(TranslocDatabase: TranslocDatabase as "Query" |&self| {
    description: "The root query of the Transloc graphql database"
        
    field route(&executor, id: i32 as "route id (required)") -> Option<&Route> {
        executor.context().get_route(&id)
    }

    field routes(&executor) -> Vec<&Route> {
        executor.context().get_routes()
    }

    field stop(&executor, id: i32 as "stop id (required)") -> Option<&Stop> {
        executor.context().get_stop(&id)
    }

    field stops(&executor) -> Vec<&Stop> {
        executor.context().get_stops()
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
