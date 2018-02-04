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

/// Lists all the periods
#[get("/")]
pub fn index(message: Option<FlashMessage>, conn: db::PgSqlConn) -> Template {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let periods = models::period::Period::list(&conn);
    match periods {
        Ok(periods) => {
            let context = IndexTemplateContext {
                model: periods,
                flash: flash,
                extra_data: (),
            };
            Template::render("period/index", &context)
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
    let period = match id {
        0 => Ok(models::period::Period::new()),
        _ => models::period::Period::get(id, &conn),
    };
    match period {
        Ok(period) => {
            let context = TemplateContext {
                model: period,
                flash: flash,
                extra_data: (),
            };
            Template::render("period/edit", &context)
        }
        Err(e) => {
            error_page(e)
        }
    }
}

#[post("/<id>/edit", data = "<period_form>")]
pub fn edit_post(
    id: u32,
    period_form: Form<models::period::PeriodForm>,
    conn: db::PgSqlConn,
) -> Result<Flash<Redirect>, Template> {
    let period = period_form.into_inner();
    if period.name.is_empty() {
        Ok(Flash::error(
            Redirect::to(&format!("/periods/{0}/edit", id)),
            "Name cannot be empty",
        ))
    } else {
        match period.save(&conn) {
            Ok(_) => Ok(Flash::success(Redirect::to("/periods"), "Period saved.")),
            Err(e) => {
                Err(error_page(e))
            }
        }
    }
}
