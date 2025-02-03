// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (id) {
        id -> Integer,
        website -> Text,
        username -> Text,
        password -> Text,
    }
}
