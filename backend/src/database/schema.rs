// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Integer,
        name -> Tinytext,
        email -> Tinytext,
        password -> Text,
    }
}

diesel::table! {
    album (id) {
        id -> Integer,
        name -> Tinytext,
        description -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Tinytext,
        email -> Tinytext,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    album,
    user,
);
