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
use views::transaction::TransactionIndex;

/// Returns all the routes defined on this controller
pub fn all_routes() -> Vec<rocket::Route> {
    routes![index, edit, save]
}

/// Renders the edit view for a given transaction
fn edit_view<T>(
    transaction_form: T,
    conn: &db::PgSqlConn,
    flash: Option<String>,
) -> response::Response<()>
where
    T: Into<models::transaction::TransactionForm>,
{
    use models::period::Period;
    use models::category::Category;

    let _periods = Period::list(conn);
    let _categories = Category::list(conn);

    let transaction_form = transaction_form.into();
    let title: Cow<'static, str> = if transaction_form.is_new() {
        "New Transaction".into()
    } else {
        transaction_form.description.clone().into()
    };

    let context = TemplateContext {
        model: transaction_form,
        flash: flash,
        title: title,
        extra_data: (),
    };
    view("transaction/edit", &context)
}

/// Lists all the transactions
#[get("/")]
pub fn index(
    message: Option<FlashMessage>,
    conn: db::PgSqlConn,
) -> response::Response<TransactionIndex> {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let transactions = models::transaction::Transaction::list(&conn);
    match transactions {
        Ok(transactions) => {
            let context = TransactionIndex {
                transactions: transactions,
                flash: flash,
                title: "Transactions".into(),
            };
            template(context)
        }
        Err(e) => error(e),
    }
}

/// Returns the edit page for the given transaction (including new transactions)
#[get("/<id>/edit")]
pub fn edit(id: i32, conn: db::PgSqlConn, message: Option<FlashMessage>) -> response::Response<()> {
    let flash = if let Some(message) = message {
        Some(message.msg().to_string())
    } else {
        None
    };
    let transaction = match id {
        0 => Ok(models::transaction::Transaction::new()),
        _ => models::transaction::Transaction::get(id, &conn),
    };
    match transaction {
        Ok(transaction) => edit_view(transaction, &conn, flash),
        Err(e) => error(e),
    }
}

/// Handles the save / edit of a transaction, includes validation
#[post("/<_id>/edit", data = "<transaction_form>")]
pub fn save(
    _id: u32,
    transaction_form: Form<models::transaction::TransactionForm>,
    conn: db::PgSqlConn,
) -> response::Response<()> {
    let transaction_form = transaction_form.into_inner();
    let is_valid = transaction_form.is_valid();
    match is_valid {
        ValidateResult::Invalid(_) => {
            edit_view(transaction_form, &conn, Some(String::from(is_valid)))
        }
        ValidateResult::Valid => match transaction_form.save(&conn) {
            Ok(_) => saved("/transactions"),
            Err(e) => error(e),
        },
    }
}
