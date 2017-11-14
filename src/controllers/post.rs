use rocket_contrib::Template;
use rocket;
use rocket::request::{LenientForm, FlashMessage};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use markdown;
use db;
use models;
use super::context::{IndexTemplateContext, TemplateContext};

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, edit_get, edit_post, view]
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
        model: models::post::Post::list(&conn),
        flash: flash,
        extra_data: ()
    };
    Template::render("post/index", &context)
}

#[get("/<id>/edit")]
pub fn edit_get(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> Template {
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
        model: post,
        flash: flash,
        extra_data: (models::category::Category::list(&conn),)
    };
    Template::render("post/edit", &context)
}

#[post("/<_id>/edit", data = "<post_form>")]
pub fn edit_post(
    _id: u32,
    post_form: LenientForm<models::post::Post>,
    conn: db::PgSqlConn,
) -> Result<Template, Flash<Redirect>> {
    let post = post_form.into_inner();
    if post.title.is_empty() {
        let context = TemplateContext {
            model: post,
            extra_data: (models::category::Category::list(&conn),),
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