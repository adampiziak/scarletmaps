use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};

use model::transloc::{TranslocDatabase};

mod tasks;

pub fn start(transloc_db: Arc<RwLock<TranslocDatabase>> ) {
    gather_transloc_metadata(Arc::clone(&transloc_db));
    poll_transloc_arrivals(Arc::clone(&transloc_db));
    poll_transloc_vehicles(Arc::clone(&transloc_db));

    gather_transloc_segments(Arc::clone(&transloc_db));
}

fn poll_transloc_vehicles(database: Arc<RwLock<TranslocDatabase>>) {
    thread::spawn(move || {
        loop {
            tasks::update_vehicle_data(Arc::clone(&database));
            thread::sleep(Duration::from_secs(10));
        }
    });
}

fn poll_transloc_arrivals(database: Arc<RwLock<TranslocDatabase>>) {
    thread::spawn(move || {
        loop {
            tasks::update_arrival_estimates(Arc::clone(&database));
            thread::sleep(Duration::from_secs(30));
        }
    });
}

fn gather_transloc_segments(database: Arc<RwLock<TranslocDatabase>>) {
   thread::spawn(move || {
       thread::sleep(Duration::from_secs(5));
       tasks::update_segments(Arc::clone(&database));
       loop {
            tasks::update_segments(Arc::clone(&database));
            thread::sleep(Duration::from_secs(60*10));
        }
    });
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
