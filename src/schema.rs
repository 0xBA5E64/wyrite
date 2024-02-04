// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        date -> Timestamp,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}
