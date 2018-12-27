pub struct Route {
    id: i32,
    name: String,
    stops: Vec<RouteStop>
}

pub struct RouteStop {
    id: i32,
    name: String,
    campus: String,
    arrivals: Vec<f64>
}

struct Stop {
    id: i32,
    name: String,
    campus: String,
    routes: Vec<StopRoute>
}

struct StopRoute {
    id: i32,
    name: String,
    arrivals: Vec<f64>
}
