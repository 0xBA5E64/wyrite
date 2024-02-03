// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        date -> Timestamp,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}
