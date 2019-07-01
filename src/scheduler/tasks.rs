// Tasks run by the scheduler

use std::sync::{Arc, RwLock};
use api::{transloc, builder };
use api::lookup::get_route_areas;
use model::transloc::{TranslocDatabase};

// Transloc
pub fn update_route_list(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let route_list = transloc::fetch_routes().unwrap();
    builder::update_route_list(transloc_db, route_list);
}

pub fn update_stop_list(transloc_db: Arc<RwLock<TranslocDatabase>>) {
    let stop_list = transloc::fetch_stops().unwrap();
    builder::update_stop_list(Arc::clone(&transloc_db), stop_list);
    let mut db = transloc_db.write().unwrap();
    let mut stop_areas: Vec<(i32, Vec<String>)> = Vec::new();
    {
        let stops = db.get_stops();

        for stop in stops {
            let mut areas = Vec::new();
            for route_id in &stop.served_routes {
                let route = db.get_route(&route_id).unwrap();
                let route_areas = get_route_areas(&route.served_stops);
                for area in route_areas {
                    if !areas.contains(&area) {
                        areas.push(area);
                    }
                }
            }
            areas.sort();
            stop_areas.push((stop.id, areas));
        }
    }
    for stop in stop_areas {
        db.set_stop_served_areas(&stop.0, stop.1)
    }
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

}
