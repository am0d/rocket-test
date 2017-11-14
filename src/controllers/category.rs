use rocket_contrib::Template;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use db;
use models;
use super::context::{IndexTemplateContext, TemplateContext};

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, edit_get, edit_post]
}

/// Lists all the categories
#[get("/")]
pub fn index(message: Option<FlashMessage>, conn: db::PgSqlConn) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let context = IndexTemplateContext {
        model: models::category::Category::list(&conn),
        flash: flash,
        extra_data: ()
    };
    Template::render("category/index", &context)
}

#[get("/<id>/edit")]
pub fn edit_get(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let category = if id == 0 {
        models::category::Category::new()
    } else {
        models::category::Category::get(id, &conn)
    };
    let context = TemplateContext {
        model: category,
        flash: flash,
        extra_data: ()
    };
    Template::render("category/edit", &context)
}

#[post("/<id>/edit", data = "<category_form>")]
pub fn edit_post(
    id: u32,
    category_form: Form<models::category::Category>,
    conn: db::PgSqlConn,
) -> Flash<Redirect> {
    let category = category_form.into_inner();
    if category.name.is_empty() {
        Flash::error(Redirect::to(&format!("/categories/{0}/edit", id)), "Name cannot be empty")
    } else if category.save(&conn) {
        Flash::success(Redirect::to("/categories"), "Category saved.")
    } else {
        Flash::error(
            Redirect::to("/categories/new"),
            "Saving is not yet implemented, sorry",
        )
    }
}