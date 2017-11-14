#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "clippy", allow(suspicious_else_formatting,needless_pass_by_value))]

#[macro_use]
extern crate diesel_codegen;

pub mod schema;
pub mod db;
pub mod models;
pub mod controllers;

extern crate rocket_contrib;
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate markdown;

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
