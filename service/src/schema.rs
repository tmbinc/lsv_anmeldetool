// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
        begin -> Nullable<Date>,
    }
}

diesel::table! {
    group_event (rowid) {
        rowid -> Integer,
        group_id -> Text,
        org_id -> Text,
    }
}

diesel::table! {
    groups (id) {
        id -> Text,
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
        contact_phone -> Nullable<Text>,
    }
}

diesel::table! {
    teams (id) {
        id -> Text,
        event -> Text,
        org -> Text,
        name -> Text,
    }
}

diesel::joinable!(org_event -> events (event_id));
diesel::joinable!(org_event -> orgs (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    group_event,
    groups,
    org_event,
    orgs,
    teams,
);
