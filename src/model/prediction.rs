pub struct RoutePrediction {
    pub id: String,
    pub active: bool,
    pub stops: Vec<RouteStopPrediction>
}

pub struct RouteStopPrediction {
    pub id: String,
    pub arrivals: Vec<f64>
}

pub struct StopPrediction {
    pub id: String,
    pub active: bool,
    pub routes: Vec<StopRoutePrediction>
}

pub struct StopRoutePrediction {
    pub id: String,
    pub arrivals: Vec<f64>
}

impl RoutePrediction {
    pub fn new(id: String) -> RoutePrediction {
        RoutePrediction {
            id,
            active: false,
            stops: Vec::new()
        }
    }
}

impl StopPrediction {
    pub fn new(id: String) -> StopPrediction {
        StopPrediction {
            id,
            active: false,
            routes: Vec::new()
        }            
    }
}
