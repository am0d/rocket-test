#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

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
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate markdown;

use rocket_contrib::Template;
use rocket::request::FlashMessage;
use rocket::response::Redirect;

#[derive(Serialize)]
pub struct IndexTemplateContext {
    posts: Vec<models::post::Post>,
    flash: Option<String>,
}

#[get("/")]
fn index(message: Option<FlashMessage>, conn: db::PgSqlConn) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let context = IndexTemplateContext {
        posts: models::post::Post::list(&conn),
        flash: flash,
    };
    Template::render("index", &context)
}

fn main() {
    let instance = rocket::ignite()
        .manage(db::establish_connection())
        .mount("/", routes![index])
        .attach(Template::fairing());
    let instance = controllers::mount(instance);
    instance.launch();
}
