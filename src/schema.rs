// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Text>,
        name -> Text,
        email -> Text,
        hashed_password -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
