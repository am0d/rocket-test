use rocket_contrib::Template;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect, Response};
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
    categories: Option<Vec<models::category::Category>>,
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
        categories: Some(models::category::Category::list(&conn)),
        flash: flash,
    };
    Template::render("post/edit", &context)
}

#[post("/<_id>/edit", data = "<post_form>")]
pub fn new_post_post(
    _id: u32,
    post_form: Form<models::post::Post>,
    conn: db::PgSqlConn,
) -> Result<Template, Flash<Redirect>> {
    let post = post_form.into_inner();
    if post.title.is_empty() {
        let context = TemplateContext {
            post: post,
            categories: Some(models::category::Category::list(&conn)),
            flash: Some("Title cannot be empty".to_string()),
        };
        Ok(Template::render("post/edit", &context))
    } else if post.save(&conn) {
        Err(Flash::success(Redirect::to("/posts"), "Post saved."))
    } else {
        Err(Flash::error(
            Redirect::to("/posts/new"),
            "Saving is not yet implemented, sorry",
        ))
    }
}

#[get("/<id>")]
pub fn view(id: i32, conn: db::PgSqlConn) -> Template {
    let post = models::post::Post::get(id, &conn);
    markdown::to_html(&post.body);
    Template::render("post/view", &post)
}