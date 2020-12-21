table! {
    links (id) {
        id -> Int4,
        shortened -> Varchar,
        original -> Text,
        created_at -> Timestamp,
    }
}
