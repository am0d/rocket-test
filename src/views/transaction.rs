use askama::Template; // bring trait in scope

#[derive(Template)]
#[template(path = "transaction/index.html")]
pub struct TransactionIndex {
    title: String,
    transactions: Vec<super::super::models::transaction::Transaction>,
    flash: Option<String>,
}
