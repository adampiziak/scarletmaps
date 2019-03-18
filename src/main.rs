#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_cors;
#[macro_use] extern crate phf;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate juniper_rocket;
extern crate chrono;
extern crate polyline;

mod api;
mod model;
mod scheduler;

use rocket::{State, Request, Response};
use std::io::Cursor;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method, Status};
use rocket::response::content;
use model::nextbus::NextBusDatabase;
use model::transloc::TranslocDatabase;
use std::sync::{Arc, RwLock};
use juniper::{EmptyMutation, RootNode};

type NextBusSchema = RootNode<'static, NextBusDatabase, EmptyMutation<NextBusDatabase>>;
type TranslocSchema = RootNode<'static, TranslocDatabase, EmptyMutation<TranslocDatabase>>;

#[get("/")]
fn root() -> &'static str {
    "OK"
}

#[get("/graphql")]
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

#[options("/graphql")]
fn options_graphql() -> Status {
    Status::Ok
}

#[post("/graphql", data = "<request>")]
fn post_graphql(context: State<Arc<RwLock<TranslocDatabase>>>,
                request: juniper_rocket::GraphQLRequest,
                schema: State<TranslocSchema>)
                -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}

/*

#[get("/graphql?<request>")]
fn graphql(context: State<Arc<RwLock<NextBusDatabase>>>,
           request: juniper_rocket::GraphQLRequest,
           schema: State<NextBusSchema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}

#[options("/graphql")]
fn options_graphql() -> Status {
    Status::Ok
}

#[post("/graphql", data = "<request>")]
fn post_graphql(context: State<Arc<RwLock<NextBusDatabase>>>,
                request: juniper_rocket::GraphQLRequest,
                schema: State<NextBusSchema>)
                -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}
*/

fn main() {
    
    // Create Databases
    let nextbus_database = Arc::new(RwLock::new(NextBusDatabase::new()));
    let transloc_database = Arc::new(RwLock::new(TranslocDatabase::new()));


    // Start Schedulers (Talk to nextbus and transloc servers)...
    scheduler::start(Arc::clone(&nextbus_database), Arc::clone(&transloc_database));
    rocket::ignite()
        .mount("/", routes![root, post_graphql, graphiql, graphql, options_graphql])
        .attach(CORS())
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


struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "OPTIONS, GET, POST"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Origin, Content-Type"));
         }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
