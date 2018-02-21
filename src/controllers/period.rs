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

/// Renders the edit view for a given period
fn edit_view(period_form: models::period::PeriodForm, flash: Option<String>) -> Template {
    let context = TemplateContext {
        model: period_form,
        flash: flash,
        extra_data: (),
    };
    Template::render("period/edit", &context)
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
        Err(e) => error_page(e),
    }
}

/// Returns the edit page for the given period (including new periods)
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
        Ok(period) => edit_view(models::period::PeriodForm::from(period), flash),
        Err(e) => error_page(e),
    }
}

/// Handles the save / edit of a period, includes validation
#[post("/<_id>/edit", data = "<period_form>")]
pub fn edit_post(
    _id: u32,
    period_form: Form<models::period::PeriodForm>,
    conn: db::PgSqlConn,
) -> Result<Flash<Redirect>, Template> {
    let period_form = period_form.into_inner();
    let is_valid = period_form.is_valid();
    match is_valid {
        ValidateResult::Invalid(_) => Err(edit_view(period_form, Some(String::from(is_valid)))),
        ValidateResult::Valid => match period_form.save(&conn) {
            Ok(_) => Ok(Flash::success(Redirect::to("/periods"), "Period saved.")),
            Err(e) => Err(error_page(e)),
        },
    }
}
