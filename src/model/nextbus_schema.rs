use juniper::Context;
use model::nextbus::{NextBusDatabase, Stop, StopRoute, Route, RouteStop};

impl Context for NextBusDatabase {}

graphql_object!(RouteStop: NextBusDatabase | &self | {
    field id() -> &String {
        &self.id
    }
    
    field name() -> &String {
        &self.name
    }

    field campus() -> &String {
        &self.campus
    }

    field arrivals() -> &Vec<f64> {
        &self.arrivals
    }
});

graphql_object!(Route: NextBusDatabase | &self | {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field stops(&executor, active: Option<bool>) -> Vec<&RouteStop> {
        match active {
            Some(a) => {
                if a {
                    self.stops.iter().filter(|x| x.arrivals.len() > 0).collect()
                } else {
                    self.stops.iter().filter(|x| x.arrivals.len() == 0).collect()
                }
            },
            None => self.stops.iter().map(|x| x).collect()
        }
    }
});

graphql_object!(Stop: NextBusDatabase | &self | {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field campus() -> &String {
        &self.campus
    }

    field routes(active: Option<bool>) -> Vec<&StopRoute> {
        match active {
            Some(a) => {
                if a {
                    self.routes.iter().filter(|x| x.arrivals.len() > 0).collect()
                } else {
                    self.routes.iter().filter(|x| x.arrivals.len() == 0).collect()
                }
            },
            None => self.routes.iter().map(|x| x).collect()
        }
    }
});
 
graphql_object!(StopRoute: NextBusDatabase | &self | {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field arrivals() -> &Vec<f64> {
        &self.arrivals
    }
});

graphql_object!(NextBusDatabase: NextBusDatabase as "Query" |&self| {
    description: "The root query of the NextBus graphql database"

    field routes(&executor, active: Option<bool>) -> Vec<&Route> {
        match active {
            Some(a) => executor.context().get_all().into_iter().filter(|r| r.is_active() == a).collect(),
            None => executor.context().get_all()
        }
    }

    field route(
        &executor,
        id: String as "id of the route"
    ) -> Option<&Route> {
        executor.context().get_route(&id)
    }

    field stops(&executor, active: Option<bool>) -> Vec<&Stop> {
        match active {
           Some(a) => executor.context().get_stops().into_iter().filter(|r| r.is_active() == a).collect(),
           None => executor.context().get_stops()
        }
    }

    field stop(&executor, id: String) -> Option<&Stop> {
        executor.context().get_stop(&id)
    }
});

