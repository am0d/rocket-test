use rocket_contrib::Template;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use markdown;
use db;
use models;

pub fn all_routes() -> Vec<rocket::Route> {
    routes![new_post_get, new_post_post, view]
}

#[derive(Serialize)]
pub struct TemplateContext {
    title: String,
    flash: Option<String>,
}

#[get("/new")]
pub fn new_post_get(message: Option<FlashMessage>) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let context = TemplateContext {
        title: "Hello, World".to_string(),
        flash: flash,
    };
    Template::render("post/edit", &context)
}

#[post("/", data = "<post_form>")]
pub fn new_post_post(
    post_form: Form<models::post::NewPost>,
    conn: db::PgSqlConn,
) -> Flash<Redirect> {
    let post = post_form.into_inner();
    if post.title.is_empty() {
        Flash::error(Redirect::to("/posts/new"), "Title cannot be empty")
    } else if post.insert(&conn) {
        Flash::success(Redirect::to("/"), "Post saved.")
    } else {
        Flash::error(
            Redirect::to("/posts/new"),
            "Saving is not yet implemented, sorry",
        )
    }
}

#[get("/<id>")]
pub fn view(id: i32, conn: db::PgSqlConn) -> Template {
    let post = models::post::Post::get(id, &conn);
    markdown::to_html(&post.body);
    Template::render("post/view", &post)
}