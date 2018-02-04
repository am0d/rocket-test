use rocket_contrib::Template;
use rocket;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use db;
use models;
use super::context::{IndexTemplateContext, TemplateContext};
use util::*;

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
    let categories = models::category::Category::list(&conn);
    match categories {
        Ok(categories) => {
            let context = IndexTemplateContext {
                model: categories,
                flash: flash,
                extra_data: (),
            };
            Template::render("category/index", &context)
        }
        Err(e) => {
            error_page(e)
        }
    }
}

#[get("/<id>/edit")]
pub fn edit_get(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let category = match id {
        0 => Ok(models::category::Category::new()),
        _ => models::category::Category::get(id, &conn),
    };
    match category {
        Ok(category) => {
            let context = TemplateContext {
                model: category,
                flash: flash,
                extra_data: (),
            };
            Template::render("category/edit", &context)
        }
        Err(e) => {
            error_page(e)
        }
    }
}

#[post("/<id>/edit", data = "<category_form>")]
pub fn edit_post(
    id: u32,
    category_form: Form<models::category::Category>,
    conn: db::PgSqlConn,
) -> Result<Flash<Redirect>, Template> {
    let category = category_form.into_inner();
    if category.name.is_empty() {
        Ok(Flash::error(
            Redirect::to(&format!("/categories/{0}/edit", id)),
            "Name cannot be empty",
        ))
    } else {
        match category.save(&conn) {
            Ok(_) => Ok(Flash::success(
                Redirect::to("/categories"),
                "Category saved.",
            )),
            Err(e) => {
                Err(error_page(e))
                // let context = ErrorTemplateContext::from(e);
                // Err(Template::render("error", &context))
            }
        }
    }
}
