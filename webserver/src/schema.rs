// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        description -> Text,
        reward -> Int8,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    users,
);
