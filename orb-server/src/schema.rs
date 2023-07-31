// @generated automatically by Diesel CLI.

diesel::table! {
    sites (id) {
        id -> Binary,
        module_id -> Binary,
        name -> Text,
    }
}
