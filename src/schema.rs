// @generated automatically by Diesel CLI.

diesel::table! {
    like (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        reaction -> Bool,
    }
}

diesel::table! {
    post (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        date -> Timestamp,
        user_id -> Nullable<Int4>,
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

diesel::joinable!(like -> post (post_id));
diesel::joinable!(like -> user_table (user_id));
diesel::joinable!(post -> user_table (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    like,
    post,
    user_table,
);
