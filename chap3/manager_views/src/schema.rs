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
    }
}

diesel::allow_tables_to_appear_in_same_query!(other_to_do_items, to_do,);
