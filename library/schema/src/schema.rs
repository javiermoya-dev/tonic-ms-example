// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        #[max_length = 80]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        max_capacity_day -> Varchar,
        most_liked -> Nullable<Decimal>,
        enabled_for_menu -> Nullable<Tinyint>,
        is_active -> Nullable<Integer>,
        restricted_18 -> Nullable<Tinyint>,
        estimated_minutes_item_preparation -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        store_id -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        #[max_length = 255]
        login_session -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    items,
    users,
);
