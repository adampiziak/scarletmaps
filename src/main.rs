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


mod api;
mod model;
mod scheduler;

use rocket::{State};
use rocket::response::content;
use model::Database;
use std::sync::{Arc, RwLock};
use juniper::{EmptyMutation, RootNode};

type Schema = RootNode<'static, Database, EmptyMutation<Database>>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn graphql(context: State<Arc<RwLock<Database>>>,
           request: juniper_rocket::GraphQLRequest,
           schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}

#[post("/graphql", data = "<request>")]
fn post_graphql(context: State<Arc<RwLock<Database>>>,
                request: juniper_rocket::GraphQLRequest,
                schema: State<Schema>)
                -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context.read().unwrap())
}

fn main() {
    let database = Arc::new(RwLock::new(Database::new()));
    scheduler::start(Arc::clone(&database));
    rocket::ignite()
        .mount("/", routes![post_graphql, graphiql, graphql])
        .manage(Arc::clone(&database))
        .manage(Schema::new(
            Database::new(),
            EmptyMutation::<Database>::new(),
        ))
        .launch();
}
