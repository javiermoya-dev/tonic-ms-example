// @generated automatically by Diesel CLI.

diesel::table! {
    movies (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        year -> Integer,
        #[max_length = 255]
        genre -> Varchar,
        #[max_length = 255]
        rating -> Varchar,
        cast -> Text,
        #[max_length = 255]
        image -> Varchar,
    }
}
