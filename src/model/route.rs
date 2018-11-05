#[derive(Serialize)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub active: bool
}

impl Route {
    pub fn new(id: String, name: String) -> Route {
        Route { id, name, active: false }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
