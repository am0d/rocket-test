use schema::transaction;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::prelude::*;
use util::*;
use models::prelude::*;
use super::NewTransaction;

#[derive(Identifiable, Insertable, Debug, Clone, AsChangeset, Queryable, Serialize)]
#[table_name = "transaction"]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub transaction_date: Option<NaiveDate>,
    pub amount: Cents,
    pub period_id: i32,
    pub category_id: i32,
}


impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            id: 0,
            description: String::new(),
            transaction_date: Some(Utc::now().naive_utc().date()),
            amount: 0,
            period_id: 0,
            category_id: 0,
        }
    }

    pub fn get_by_period(period_id: i32, conn: &PgConnection) -> AppResult<Vec<Transaction>> {
        transaction::table
            .filter(transaction::period_id.eq(period_id))
            .load::<Transaction>(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }
}

impl_crud!(Transaction, NewTransaction, transaction);
