// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Int4,
        #[max_length = 255]
        brand -> Varchar,
        #[max_length = 255]
        model -> Varchar,
        #[max_length = 6]
        year -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
