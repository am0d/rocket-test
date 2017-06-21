#![feature(plugin,custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;

#[derive(Serialize)]
pub struct TemplateContext {
    title: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { title: "Hello, World".to_string() };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}