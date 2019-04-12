// Tasks run by the scheduler

use std::sync::{Arc, RwLock};
use api::{transloc, builder };
use model::transloc::{TranslocDatabase};

// Transloc
pub fn update_routes_via_transloc(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let routes = transloc::fetch_routes().unwrap();
    builder::update_route_list(transloc_db, routes);
}

pub fn update_stops_via_transloc(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let stops = transloc::fetch_stops().unwrap();
    builder::update_stop_list(transloc_db, stops);
}

pub fn update_arrival_estimates(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let estimates = transloc::fetch_arrival_estimates().unwrap();
    builder::update_arrival_estimates(transloc_db, estimates);
}

pub fn update_segments(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let mut db = transloc_db.write().unwrap();
    let mut ids = Vec::new();
    {
        let routes = db.get_routes();

        for route in routes {
            ids.push(route.id);
        }
    }

    for id in ids {
        let segments = transloc::fetch_segments(id).unwrap();
        if let Some(r) = db.routes.get_mut(&id) {
            r.segments = segments;
        }
    }
}

pub fn update_vehicle_data(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let vehicles = transloc::fetch_vehicle_locations().unwrap().data.vehicles;
    builder::update_vehicle_data(transloc_db, vehicles);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
