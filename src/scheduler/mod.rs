use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};

use model::nextbus::{NextBusDatabase};
use model::transloc::{TranslocDatabase};

mod tasks;

pub fn start(nextbus_db: Arc<RwLock<NextBusDatabase>>, transloc_db: Arc<RwLock<TranslocDatabase>> ) {
    gather_transloc_metadata(Arc::clone(&transloc_db));

    start_config_updater(Arc::clone(&nextbus_db));
    start_prediction_updater(Arc::clone(&nextbus_db));
}

fn gather_transloc_metadata(database: Arc<RwLock<TranslocDatabase>>) {
    thread::spawn(move || {
        loop {
            tasks::update_routes_via_transloc(Arc::clone(&database));
            tasks::update_stops_via_transloc(Arc::clone(&database));
            thread::sleep(Duration::from_secs(60*5));
        }
    });
}

fn start_config_updater(database: Arc<RwLock<NextBusDatabase>>) {
    thread::spawn(move || {
        loop {
            tasks::update_route_config(Arc::clone(&database)).unwrap();
            thread::sleep(Duration::from_secs(60*5));
        }
    });
}

fn start_prediction_updater(database: Arc<RwLock<NextBusDatabase>>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        loop {
            tasks::update_route_predictions(Arc::clone(&database));
            thread::sleep(Duration::from_secs(10));
        }
    });
}


