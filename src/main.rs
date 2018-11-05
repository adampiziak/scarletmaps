#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod api;
mod model;
mod scheduler;

use rocket::State;
use model::Database;
use std::sync::{Arc, RwLock};

#[get("/<route_id>")]
fn config(database: State<Arc<RwLock<Database>>>, route_id: String) -> String {
    serde_json::to_string(database.read().unwrap().get_route(&route_id).unwrap()).unwrap()
}

fn main() {
    let database = Arc::new(RwLock::new(Database::new()));
    scheduler::start(Arc::clone(&database));
    rocket::ignite()
        .mount("/", routes![config])
        .manage(Arc::clone(&database))
        .launch();
}
