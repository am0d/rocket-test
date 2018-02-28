//! Diesel generated database schema

table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    period (id) {
        id -> Int4,
        name -> Varchar,
        start_date -> Date,
        end_date -> Nullable<Date>,
        previous_period_id -> Nullable<Int4>,
    }
}

table! {
    periodcategory (period_id, category_id) {
        period_id -> Int4,
        category_id -> Int4,
        budgeted_amount -> Int4,
        remaining_amount -> Int4,
    }
}

table! {
    transaction (id) {
        id -> Int4,
        description -> Varchar,
        transaction_date -> Nullable<Date>,
        amount -> Int4,
        period_id -> Int4,
        category_id -> Int4,
    }
}

joinable!(periodcategory -> category (category_id));
joinable!(periodcategory -> period (period_id));
joinable!(transaction -> category (category_id));
joinable!(transaction -> period (period_id));

allow_tables_to_appear_in_same_query!(category, period, periodcategory, transaction,);
