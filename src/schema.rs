// @generated automatically by Diesel CLI.

diesel::table! {
    orgs (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
        contact_email -> Nullable<Text>,
        contact_phone -> Nullable<Text>,
    }
}
