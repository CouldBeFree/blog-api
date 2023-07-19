// @generated automatically by Diesel CLI.

diesel::table! {
    user_table (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
