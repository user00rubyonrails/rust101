// @generated automatically by Diesel CLI.

diesel::table! {
    other_to_do_items (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    to_do (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        unique_id -> Varchar,
    }
}

diesel::joinable!(to_do -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(other_to_do_items, to_do, users,);
