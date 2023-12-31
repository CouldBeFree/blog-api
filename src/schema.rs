// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        content -> Varchar,
        date -> Timestamp,
    }
}

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
        title -> Varchar,
        content -> Varchar,
        date -> Timestamp,
        user_id -> Int4,
        // positive_counter -> Int4,
        // negative_counter -> Int4,
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

diesel::joinable!(comment -> post (post_id));
diesel::joinable!(comment -> user_table (user_id));
diesel::joinable!(like -> post (post_id));
diesel::joinable!(like -> user_table (user_id));
diesel::joinable!(post -> user_table (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    like,
    post,
    user_table,
);
