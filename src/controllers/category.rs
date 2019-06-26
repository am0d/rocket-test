use std::borrow::Cow;
use rocket;
use rocket::request::{FlashMessage, Form};
use std::vec::Vec;
use db;
use models;
use super::context::{IndexTemplateContext, TemplateContext};
use util::*;
use util::response::*;
use models::prelude::*;

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, edit_get, edit_post]
}

/// Returns the edit view for a given category
fn edit_view<M>(category_form: M, flash: Option<String>) -> response::Response<()>
where
    M: Into<models::category::CategoryForm>,
{
    let category_form = category_form.into();
    let title: Cow<'static, str> = if category_form.is_new() {
        "New Category".into()
    } else {
        category_form.name.clone().into()
    };
    let context = TemplateContext {
        model: category_form,
        flash: flash,
        title: title,
        extra_data: (),
    };
    view("category/edit", &context)
}

/// Lists all the categories
#[get("/")]
pub fn index(message: Option<FlashMessage>, conn: db::PgSqlConn) -> response::Response<()> {
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
                title: "Categories".into(),
                extra_data: (),
            };
            view("category/index", &context)
        }
        Err(e) => error(e),
    }
}

#[get("/<id>/edit")]
pub fn edit_get(
    id: i32,
    conn: db::PgSqlConn,
    message: Option<FlashMessage>,
) -> response::Response<()> {
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
        Ok(category) => edit_view(category, flash),
        Err(e) => error(e),
    }
}

#[post("/<_id>/edit", data = "<category_form>")]
pub fn edit_post(
    _id: u32,
    category_form: Form<models::category::CategoryForm>,
    conn: db::PgSqlConn,
) -> response::Response<()> {
    let category_form = category_form.into_inner();
    let is_valid = category_form.is_valid();
    match is_valid {
        ValidateResult::Invalid(_) => edit_view(category_form, Some(String::from(is_valid))),
        ValidateResult::Valid => match category_form.save(&conn) {
            Ok(_) => saved("/categories"),
            Err(e) => error(e),
        },
    }
}
