#[derive(Serialize)]
pub struct Route {
    pub id: String,
    name: String,
    active: bool
}

impl Route {
    pub fn new(id: String, name: String) -> Route {
        Route { id, name, active: false }
    }
}
