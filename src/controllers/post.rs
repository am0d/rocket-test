use rocket_contrib::Template;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use markdown;
use db;
use models;

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, new_post_get, new_post_post, view]
}

#[derive(Serialize)]
pub struct IndexTemplateContext {
    posts: Vec<models::post::Post>,
    flash: Option<String>,
}

/// Lists all the posts
#[get("/")]
pub fn index(message: Option<FlashMessage>, conn: db::PgSqlConn) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let context = IndexTemplateContext {
        posts: models::post::Post::list(&conn),
        flash: flash,
    };
    Template::render("post/index", &context)
}

#[derive(Serialize)]
pub struct TemplateContext {
    post: models::post::Post,
    flash: Option<String>,
}

#[get("/<id>/edit")]
pub fn new_post_get(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let post = if id == 0 {
        models::post::Post::new()
    } else {
        models::post::Post::get(id, &conn)
    };
    let context = TemplateContext {
        post: models::post::Post::from(post),
        flash: flash,
    };
    Template::render("post/edit", &context)
}

#[post("/<id>/edit", data = "<post_form>")]
pub fn new_post_post(
    id: u32,
    post_form: Form<models::post::Post>,
    conn: db::PgSqlConn,
) -> Flash<Redirect> {
    let post = post_form.into_inner();
    if post.title.is_empty() {
        Flash::error(Redirect::to(&format!("/posts/{0}/edit", id)), "Title cannot be empty")
    } else if post.save(&conn) {
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