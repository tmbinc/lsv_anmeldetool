// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
        public_reg_until -> Nullable<Timestamp>,
        begin -> Nullable<Timestamp>,
        description -> Text,
        allow_set_present -> Bool,
        allow_user_changes -> Bool,
        results_are_public -> Bool,
        self_registration_allowed -> Bool,
    }
}

diesel::table! {
    groups (id) {
        id -> Text,
        event_id -> Text,
        name -> Text,
        replacement -> Nullable<Text>,
        slug -> Text,
        num_rounds -> Integer,
        color -> Text,
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
        slug -> Text,
    }
}

diesel::table! {
    pairings (event, round, team_home, team_guest) {
        event -> Text,
        group_id -> Text,
        round -> Integer,
        table_num -> Integer,
        team_home -> Nullable<Text>,
        team_guest -> Nullable<Text>,
        points_home -> Nullable<Integer>,
        points_guest -> Nullable<Integer>,
        result -> Nullable<Text>,
    }
}

diesel::table! {
    questionnaire (id) {
        id -> Text,
        event_id -> Nullable<Text>,
        org_id -> Nullable<Text>,
        question_text -> Text,
        question_type -> Text,
        question_data -> Text,
        sort -> Integer,
    }
}

diesel::table! {
    questionnaire_answers (question_id, event_id, org_id) {
        question_id -> Text,
        event_id -> Text,
        org_id -> Text,
        question_answer -> Text,
    }
}

diesel::table! {
    results (event, round, team) {
        event -> Text,
        group_id -> Text,
        round -> Integer,
        team -> Nullable<Text>,
        rank -> Nullable<Integer>,
        points_team -> Nullable<Integer>,
        points_player -> Nullable<Integer>,
        tie -> Nullable<Integer>,
    }
}

diesel::table! {
    rooms (event, group_id, table_num_low, table_num_high) {
        event -> Text,
        group_id -> Text,
        table_num_low -> Integer,
        table_num_high -> Integer,
        room -> Text,
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
        presence_state -> Text,
        comment -> Nullable<Text>,
        changed_since -> Nullable<Text>,
    }
}

diesel::table! {
    timetable (event, group_id, row_index) {
        event -> Text,
        group_id -> Text,
        row_index -> Integer,
        name -> Text,
        expected_time -> Timestamp,
        last_update -> Timestamp,
        state -> Text,
        flags -> Text,
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
diesel::joinable!(pairings -> groups (group_id));
diesel::joinable!(questionnaire -> events (event_id));
diesel::joinable!(questionnaire -> orgs (org_id));
diesel::joinable!(questionnaire_answers -> events (event_id));
diesel::joinable!(questionnaire_answers -> orgs (org_id));
diesel::joinable!(questionnaire_answers -> questionnaire (question_id));
diesel::joinable!(results -> groups (group_id));
diesel::joinable!(results -> teams (team));
diesel::joinable!(rooms -> groups (group_id));
diesel::joinable!(teams -> groups (group_id));
diesel::joinable!(timetable -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    groups,
    invite_queue,
    org_event,
    org_secrets,
    orgs,
    pairings,
    questionnaire,
    questionnaire_answers,
    results,
    rooms,
    teams,
    timetable,
    users,
);
