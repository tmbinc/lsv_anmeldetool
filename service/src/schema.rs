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
    invite_queue (rowid) {
        rowid -> Integer,
        event_id -> Text,
        org_id -> Text,
        created_at -> Timestamp,
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
    org_secrets (org_id, secret) {
        org_id -> Text,
        secret -> Text,
    }
}

diesel::table! {
    orgs (id) {
        id -> Text,
        name -> Text,
        name_additional -> Nullable<Text>,
        genus -> Nullable<Text>,
        public -> Bool,
        contact_email -> Nullable<Text>,
        contact_email_pending -> Nullable<Text>,
        contact_name -> Nullable<Text>,
        contact_phone -> Nullable<Text>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Text,
        event -> Text,
        org -> Text,
        name -> Text,
        present -> Bool,
        group_id -> Nullable<Text>,
        contact_name -> Nullable<Text>,
        contact_phone -> Nullable<Text>,
    }
}

diesel::table! {
    users (email) {
        email -> Text,
        hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(groups -> events (event_id));
diesel::joinable!(invite_queue -> events (event_id));
diesel::joinable!(invite_queue -> orgs (org_id));
diesel::joinable!(org_event -> events (event_id));
diesel::joinable!(org_event -> orgs (org_id));
diesel::joinable!(org_secrets -> orgs (org_id));
diesel::joinable!(teams -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    groups,
    invite_queue,
    org_event,
    org_secrets,
    orgs,
    teams,
    users,
);
