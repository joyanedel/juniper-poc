// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        hashed_password -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
