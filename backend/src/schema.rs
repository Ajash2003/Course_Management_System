// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
        code -> Varchar,
        credits -> Int4,
        department -> Varchar,
        description -> Nullable<Text>,
    }
}
