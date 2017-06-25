#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel_codegen;

pub mod schema;
pub mod db;
pub mod models;

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

use rocket_contrib::Template;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

#[derive(Serialize)]
pub struct TemplateContext {
    title: String,
    flash: Option<String>,
}

#[get("/")]
fn index(message: Option<FlashMessage>) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let context = TemplateContext {
        title: "Hello, World".to_string(),
        flash: flash,
    };
    Template::render("index", &context)
}

#[post("/", data = "<post_form>")]
fn new_post(post_form: Form<models::post::NewPost>, conn: db::PgSqlConn) -> Flash<Redirect> {
    let post = post_form.into_inner();
    if post.title.is_empty() {
        Flash::error(Redirect::to("/"), "Title cannot be empty")
    } else if post.insert(&conn) {
        Flash::success(Redirect::to("/"), "Post saved.")
    } else {
        Flash::error(Redirect::to("/"), "Saving is not yet implemented, sorry")
    }
}

fn main() {
    rocket::ignite()
        .manage(db::establish_connection())
        .mount("/", routes![index, new_post])
        .launch();
}
