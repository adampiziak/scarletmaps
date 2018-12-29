#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(phf_macros)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;

extern crate rocket;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate juniper_rocket;
extern crate phf;
extern crate chrono;

mod api;
mod model;
mod scheduler;

use rocket::{State};
use rocket::response::content;
use model::nextbus::NextBusDatabase;
use model::transloc::TranslocDatabase;
use std::sync::{Arc, RwLock};
use juniper::{EmptyMutation, RootNode};

type NextBusSchema = RootNode<'static, NextBusDatabase, EmptyMutation<NextBusDatabase>>;
type TranslocSchema = RootNode<'static, TranslocDatabase, EmptyMutation<TranslocDatabase>>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn graphql(context: State<Arc<RwLock<TranslocDatabase>>>,
           request: juniper_rocket::GraphQLRequest,
           schema: State<TranslocSchema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}

#[post("/graphql", data = "<request>")]
fn post_graphql(context: State<Arc<RwLock<TranslocDatabase>>>,
                request: juniper_rocket::GraphQLRequest,
                schema: State<TranslocSchema>)
                -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}

fn main() {
    
    // Create Databases
    let nextbus_database = Arc::new(RwLock::new(NextBusDatabase::new()));
    let transloc_database = Arc::new(RwLock::new(TranslocDatabase::new()));

    // Start Schedulers (Talk to nextbus and transloc servers)...
    scheduler::start(Arc::clone(&nextbus_database), Arc::clone(&transloc_database));
    rocket::ignite()
        .mount("/", routes![post_graphql, graphiql, graphql])
        .manage(Arc::clone(&nextbus_database))
        .manage(Arc::clone(&transloc_database))
        .manage(NextBusSchema::new(
            NextBusDatabase::new(),
            EmptyMutation::<NextBusDatabase>::new(),
        ))
        .manage(TranslocSchema::new(
            TranslocDatabase::new(),
            EmptyMutation::<TranslocDatabase>::new(),
        ))
        .launch();
}
