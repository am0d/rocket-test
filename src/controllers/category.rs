use rocket_contrib::Template;
use rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use std::vec::Vec;
use db;
use models;

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, new_category_get, new_category_post]
}

#[derive(Serialize)]
pub struct IndexTemplateContext {
    categories: Vec<models::category::Category>,
    flash: Option<String>,
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
        categories: models::category::Category::list(&conn),
        flash: flash,
    };
    Template::render("category/index", &context)
}

#[derive(Serialize)]
pub struct TemplateContext {
    category: models::category::Category,
    flash: Option<String>,
}

#[get("/<id>/edit")]
pub fn new_category_get(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> Template {
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
        category: models::category::Category::from(category),
        flash: flash,
    };
    Template::render("category/edit", &context)
}

#[post("/<id>/edit", data = "<category_form>")]
pub fn new_category_post(
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