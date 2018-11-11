pub struct Stop {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub campus: String,
    pub routes: Vec<StopRoute>
}

pub struct StopRoute {
    pub id: String,
    pub name: String,
    pub arrivals: Vec<f64>
}

impl Stop {
    pub fn new(id: String, name: String, campus: String) -> Stop {
        Stop { id, name, active: false, campus, routes: Vec::new() }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update_arrivals(&mut self, route_id: String, times: Vec<f64>) {
        self.routes.iter_mut().find(|ref mut x| x.id == route_id).unwrap().arrivals = times;
    }
}

impl StopRoute {
    pub fn new(id: String, name: String) -> StopRoute {
        StopRoute { id, name, arrivals: Vec::new() }
    }
}
