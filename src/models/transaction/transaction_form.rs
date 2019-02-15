use diesel::pg::PgConnection;
use util::*;
use models::prelude::*;
use super::Transaction;

#[derive(FromForm, Serialize, Debug)]
pub struct TransactionForm {
    pub id: i32,
    pub description: String,
    pub transaction_date: String,
    pub amount: String,
    pub period_id: i32,
    pub category_id: i32,
}

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
                    amount: (f32::from_str(&self.amount).unwrap() * 100.0) as Cents,
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
