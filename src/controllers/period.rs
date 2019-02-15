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
    routes![index, view_period, edit_get, edit_post]
}

/// Renders the edit view for a given period
fn edit_view<T>(period_form: T, flash: Option<String>) -> response::Response
where
    T: Into<models::period::PeriodForm>,
{
    let period_form = period_form.into();
    let title: Cow<'static, str> = if period_form.is_new() {
        "New Period".into()
    } else {
        period_form.name.clone().into()
    };
    let context = TemplateContext {
        model: period_form,
        flash: flash,
        title: title,
        extra_data: (),
    };
    view("period/edit", &context)
}

#[derive(Serialize)]
struct ViewTemplateContext {
    pub model: models::period::Period,
    pub transactions: Vec<models::transaction::Transaction>,
    pub title: Cow<'static, str>,
    pub flash: Option<String>,
}

/// Lists all the periods
#[get("/")]
pub fn index(message: Option<FlashMessage>, conn: db::PgSqlConn) -> response::Response {
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
                title: "Periods".into(),
                extra_data: (),
            };
            view("period/index", &context)
        }
        Err(e) => error(e),
    }
}

/// View one period
#[get("/<id>")]
pub fn view_period(
    id: i32,
    message: Option<FlashMessage>,
    conn: db::PgSqlConn,
) -> response::Response {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let period = models::period::Period::get(id, &conn).and_then(|period| {
        match period.get_transactions(&conn) {
            Ok(transactions) => Ok((period, transactions)),
            Err(err) => Err(err),
        }
    });
    match period {
        Ok((period, transactions)) => {
            let title = period.name.to_string();
            let context = ViewTemplateContext {
                model: period,
                transactions: transactions,
                flash: flash,
                title: title.into(),
            };
            view("period/view", &context)
        }
        Err(e) => error(e),
    }
}

/// Returns the edit page for the given period (including new periods)
#[get("/<id>/edit")]
pub fn edit_get(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> response::Response {
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
        Ok(period) => edit_view(period, flash),
        Err(e) => error(e),
    }
}

/// Handles the save / edit of a period, includes validation
#[post("/<_id>/edit", data = "<period_form>")]
pub fn edit_post(
    _id: u32,
    period_form: Form<models::period::PeriodForm>,
    conn: db::PgSqlConn,
) -> response::Response {
    let period_form = period_form.into_inner();
    let is_valid = period_form.is_valid();
    match is_valid {
        ValidateResult::Invalid(_) => edit_view(period_form, Some(String::from(is_valid))),
        ValidateResult::Valid => match period_form.save(&conn) {
            Ok(period) => saved(format!("/periods/{}", period.id)),
            Err(e) => error(e),
        },
    }
}
