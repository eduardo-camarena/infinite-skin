// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Tinytext,
        email -> Tinytext,
        password -> Text,
    }
}
