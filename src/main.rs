#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
#![cfg_attr(any(feature = "clippy", feature="cargo-clippy"), allow(suspicious_else_formatting, needless_pass_by_value))]

#[macro_use]
extern crate diesel_codegen;

/// Utility functions
#[macro_use]
pub mod util;
pub mod schema;
/// Database connection helper functions
pub mod db;
/// Models and database interactions
pub mod models;
/// The controllers providing the endpoints for the application
pub mod controllers;

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate markdown;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/posts")
}

fn main() {
    let instance = rocket::ignite()
        .manage(db::establish_connection())
        .mount("/", routes![index])
        .attach(Template::fairing());
    let instance = controllers::mount(instance);
    instance.launch();
}
