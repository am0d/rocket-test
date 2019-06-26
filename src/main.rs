#![feature(custom_attribute, proc_macro_hygiene, decl_macro)]
#![cfg_attr(any(feature = "clippy", feature = "cargo-clippy"),
            allow(suspicious_else_formatting, needless_pass_by_value))]

#[macro_use]
pub mod util;
pub mod schema;
pub mod db;
pub mod models;
pub mod controllers;
pub mod views;

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate failure;
//#[macro_use]
extern crate askama;
extern crate failure_derive;
extern crate markdown;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::templates::Template;
use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/periods")
}

fn main() {
    let instance = rocket::ignite()
        .manage(db::establish_connection())
        .mount("/", routes![index])
        .attach(Template::fairing());
    let instance = controllers::mount(instance);
    instance.launch();
}
