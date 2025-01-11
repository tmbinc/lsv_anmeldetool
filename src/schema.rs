// @generated automatically by Diesel CLI.

diesel::table! {
    orgs (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
    }
}
