diesel::table! {
    tokens (id) {
        id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        expires -> Timestamp
    }
}