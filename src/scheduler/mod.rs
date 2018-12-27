use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};

use reqwest::Client;
use model::database::{Database, SyncedDatabase};
use api;

mod tasks;

pub fn start(database: Arc<RwLock<Database>>) {
    gather_transloc_metadata(Arc::clone(&database));
    start_config_updater(Arc::clone(&database));
    start_prediction_updater(Arc::clone(&database));
}

fn gather_transloc_metadata(database: SyncedDatabase) {
    thread::spawn(move || {
        loop {
            tasks::update_routes_via_transloc(Arc::clone(&database));
            tasks::update_stops_via_transloc(Arc::clone(&database));
            thread::sleep(Duration::from_secs(60*5));
        }
    });
}

fn start_config_updater(database: Arc<RwLock<Database>>) {
    thread::spawn(move || {
        loop {
            tasks::update_route_config(Arc::clone(&database)).unwrap();
            thread::sleep(Duration::from_secs(60*5));
        }
    });
}

fn start_prediction_updater(database: Arc<RwLock<Database>>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        loop {
            tasks::update_route_predictions(Arc::clone(&database));
            thread::sleep(Duration::from_secs(10));
        }
    });
}


