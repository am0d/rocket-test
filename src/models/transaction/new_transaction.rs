use schema::transaction;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::prelude::*;
use util::*;
use super::Transaction;

#[derive(Insertable, Debug)]
#[table_name = "transaction"]
pub struct NewTransaction {
    pub description: String,
    pub transaction_date: Option<NaiveDate>,
    pub amount: Cents,
    pub period_id: i32,
    pub category_id: i32,
}

impl NewTransaction {
    pub fn insert(&self, conn: &PgConnection) -> AppResult<Transaction>
    where
        Self: Sized,
    {
        diesel::insert_into(transaction::table)
            .values(self)
            .get_result(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }
}

impl<'a> From<&'a Transaction> for NewTransaction {
    fn from(transaction: &'a Transaction) -> NewTransaction {
        NewTransaction {
            description: transaction.description.clone(),
            transaction_date: transaction.transaction_date,
            amount: transaction.amount,
            period_id: transaction.period_id,
            category_id: transaction.category_id
        }
    }
}