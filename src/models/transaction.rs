/// Individual budget transactions.

use schema::transaction;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::prelude::*;
use util::*;
use models::prelude::*;

#[derive(Identifiable, Insertable, Debug, Clone, AsChangeset, Queryable, Serialize)]
#[table_name = "transaction"]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub transaction_date: Option<NaiveDate>,
    pub amount: i32, // TODO this should be a special Cents money type
    pub period_id: i32,
    pub category_id: i32,
}

#[derive(FromForm, Serialize, Debug)]
pub struct TransactionForm {
    pub id: i32,
    pub description: String,
    pub transaction_date: String,
    pub amount: String,
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

impl_crud!(Transaction, transaction);

impl TransactionForm {
    pub fn is_new(&self) -> bool {
        self.id == 0
    }
    
    pub fn save(&self, conn: &PgConnection) -> AppResult<Transaction> {
        use std::str::FromStr;
        match self.is_valid() {
            ValidateResult::Valid => {
                let transaction = Transaction {
                    id: self.id,
                    description: self.description.clone(),
                    transaction_date: date_from_str(&self.transaction_date).ok(),
                    amount: (f32::from_str(&self.amount).unwrap() * 100.0) as i32,
                    period_id: self.period_id,
                    category_id: self.category_id,
                };
                transaction.save(conn)
            }
            errors @ ValidateResult::Invalid(_) => {
                Err(errors::AppError::ValidationError { errors })
            }
        }
    }
}

impl From<Transaction> for TransactionForm {
    fn from(transaction: Transaction) -> TransactionForm {
        TransactionForm {
            id: transaction.id,
            description: transaction.description,
            transaction_date: transaction
                .transaction_date
                .map(|td| td.to_string())
                .unwrap_or("".into()),
            amount: format!("{}", transaction.amount),
            period_id: transaction.period_id,
            category_id: transaction.category_id,
        }
    }
}

impl Validate for TransactionForm {
    fn is_valid(&self) -> ValidateResult {
        let mut errors = vec![];
        if self.description.is_empty() {
            errors.push("Description is required".into());
        }
        if date_from_str(&self.transaction_date).is_err() {
            errors.push("Transaction date is required".into());
        }
        match errors.len() {
            0 => ValidateResult::Valid,
            _ => ValidateResult::Invalid(errors),
        }
    }
}
