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
    post (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
