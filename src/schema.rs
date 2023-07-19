// @generated automatically by Diesel CLI.

diesel::table! {
    post (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        date -> Timestamp,
    }
}

diesel::table! {
    user_table (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    post,
    user_table,
);
