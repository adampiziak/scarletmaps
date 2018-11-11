use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};

use model::{ Database };

mod tasks;

pub fn start(database: Arc<RwLock<Database>>) {
    start_config_updater(Arc::clone(&database));
    start_prediction_updater(Arc::clone(&database));
}

fn start_config_updater(database: Arc<RwLock<Database>>) {
    thread::spawn(move || {
        loop {
            tasks::update_route_config(Arc::clone(&database)).unwrap();
            thread::sleep(Duration::from_secs(60));
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


