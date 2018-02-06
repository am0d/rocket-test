/// Individual budget transactions.

use schema::transaction;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::prelude::*;
use util::*;
use models::crud::Crud;

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "transaction"]
pub struct NewTransaction {
    description: String,
    transaction_date: Option<NaiveDateTime>,
    pub amount: i32, // TODO this should be a special Cents money type
    pub period_id: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(Identifiable, Insertable, Debug, Clone, AsChangeset, Queryable, Serialize)]
#[table_name = "transaction"]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub transaction_date: Option<NaiveDateTime>,
    pub amount: i32, // TODO this should be a special Cents money type
    pub period_id: i32,
    pub category_id: i32,
}

impl Transaction {
    pub fn new() -> NewTransaction {
        NewTransaction {
            description: String::new(),
            transaction_date: Some(Utc::now().naive_utc()),
            amount: 0,
            period_id: None,
            category_id: None,
        }
    }
}

impl_crud!(Transaction, transaction);