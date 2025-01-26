// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
        public_reg_until -> Nullable<Timestamp>,
        begin -> Nullable<Timestamp>,
        description -> Text,
    }
}

diesel::table! {
    groups (id) {
        id -> Text,
        event_id -> Text,
        name -> Text,
    }
}

diesel::table! {
    org_event (event_id, org_id) {
        event_id -> Text,
        org_id -> Text,
        state -> Text,
    }
}

diesel::table! {
    orgs (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
        contact_email -> Nullable<Text>,
        contact_name -> Nullable<Text>,
        contact_phone -> Nullable<Text>,
        confirmed_email -> Bool,
    }
}

diesel::table! {
    teams (id) {
        id -> Text,
        event -> Text,
        org -> Text,
        name -> Text,
        group_id -> Nullable<Text>,
        contact_name -> Nullable<Text>,
        contact_phone -> Nullable<Text>,
    }
}

diesel::joinable!(groups -> events (event_id));
diesel::joinable!(org_event -> events (event_id));
diesel::joinable!(org_event -> orgs (org_id));
diesel::joinable!(teams -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    groups,
    org_event,
    orgs,
    teams,
);
