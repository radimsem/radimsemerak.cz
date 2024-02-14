diesel::table! {
    projects (id) {
        id -> Int4,
        html -> Text,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        expires -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tokens,
);
