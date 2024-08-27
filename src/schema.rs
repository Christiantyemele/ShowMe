// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (session_token) {
        user_id -> Nullable<Int4>,
        session_token -> Bytea,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passkey -> Varchar,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
