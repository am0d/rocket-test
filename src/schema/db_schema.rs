/// Diesel generated database schema

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
    periodcategory (periodid, categoryid) {
        periodid -> Int4,
        categoryid -> Int4,
        budgetedamount -> Int4,
        remainingamount -> Int4,
    }
}

table! {
    transaction (id) {
        id -> Int4,
        description -> Varchar,
        transactiondate -> Nullable<Timestamp>,
        amount -> Int4,
        periodid -> Int4,
        categoryid -> Int4,
    }
}

joinable!(periodcategory -> category (categoryid));
joinable!(periodcategory -> period (periodid));
joinable!(transaction -> category (categoryid));
joinable!(transaction -> period (periodid));

allow_tables_to_appear_in_same_query!(
    category,
    period,
    periodcategory,
    transaction,
);
