use rocket_contrib::Template;
use rocket;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use db;
use models;
use super::context::{IndexTemplateContext, TemplateContext};
use util::*;
use models::prelude::*;

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, edit_get, edit_post]
}

/// Returns the edit view for a given category
fn edit_view(category_form: models::category::CategoryForm, flash: Option<String>) -> Template {
    let context = TemplateContext {
        model: category_form,
        flash: flash,
        extra_data: (),
    };
    Template::render("category/edit", &context)
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
        Err(e) => error_page(e),
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
        Ok(category) => edit_view(models::category::CategoryForm::from(category), flash),
        Err(e) => error_page(e),
    }
}

#[post("/<_id>/edit", data = "<category_form>")]
pub fn edit_post(
    _id: u32,
    category_form: Form<models::category::CategoryForm>,
    conn: db::PgSqlConn,
) -> Result<Flash<Redirect>, Template> {
    let category_form = category_form.into_inner();
    let is_valid = category_form.is_valid();
    match is_valid {
        ValidateResult::Invalid(_) => Err(edit_view(category_form, Some(String::from(is_valid)))),
        ValidateResult::Valid => match category_form.save(&conn) {
            Ok(_) => Ok(Flash::success(
                Redirect::to("/categories"),
                "Category saved.",
            )),
            Err(e) => Err(error_page(e)),
        },
    }
}
