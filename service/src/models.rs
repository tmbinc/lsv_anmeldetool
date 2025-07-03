use crate::schema::{
    events, groups, invite_queue, org_event, org_secrets, orgs, pairings, questionnaire,
    questionnaire_answers, results, rooms, teams, timetable, users,
};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Org details.
#[derive(
    Debug,
    Clone,
    Serialize,
    Selectable,
    Deserialize,
    Queryable,
    Insertable,
    JsonSchema,
    ApiComponent,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = orgs)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub name_additional: Option<String>,
    pub genus: Option<String>,
    pub public: bool,
    pub contact_email: Option<String>,
    pub contact_email_pending: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub last_update: NaiveDateTime,
    pub slug: String,
}

/// New org details.
#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct NewOrg {
    pub name: String,
}

impl NewOrg {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Team details.
#[derive(
    Debug,
    Clone,
    Serialize,
    Selectable,
    Deserialize,
    Queryable,
    Insertable,
    ApiComponent,
    JsonSchema,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: String,
    pub event: String,
    pub org: String,
    pub name: String,
    pub group_id: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub presence_state: String,
    pub comment: Option<String>,
    pub changed_since: Option<String>,
}

/// New org details.
#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct NewTeam {
    pub name: String,
    pub event: String,
    pub org: String,
}

impl NewTeam {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>, event: impl Into<String>, org: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            event: event.into(),
            org: org.into(),
        }
    }
}

/// Event details
#[derive(
    Debug,
    Clone,
    Serialize,
    Selectable,
    Deserialize,
    Queryable,
    Insertable,
    JsonSchema,
    ApiComponent,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = events)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub public_reg_until: Option<NaiveDateTime>,
    pub begin: Option<NaiveDateTime>,
    pub description: String,
    pub allow_set_present: bool,
    pub allow_user_changes: bool,
    pub results_are_public: bool,
    pub self_registration_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub enum EventOrgState {
    /// Org has not been included in this event.
    NotEnlisted,
    /// Org (self-)registered for this event, but no invitation sent.
    Registered,
    /// Org was confirmed (or manually added) to the event, was sent invitation
    Invited,
    /// Org has done any changes (i.e. clicked on final email link)
    Updated,
    /// Org has finalized entries for this event.
    Submitted,
    /// Org has been verified by admin
    Verified,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct EventOrgStateUpdate {
    pub state: EventOrgState,
}

impl EventOrgState {
    pub fn to_db(&self) -> &'static str {
        match self {
            Self::NotEnlisted => "not_enlisted",
            Self::Registered => "registered",
            Self::Invited => "invited",
            Self::Updated => "updated",
            Self::Submitted => "submitted",
            Self::Verified => "verified",
        }
    }

    #[allow(unused)]
    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "not_enlisted" => Some(Self::NotEnlisted),
            "registered" => Some(Self::Registered),
            "invited" => Some(Self::Invited),
            "updated" => Some(Self::Updated),
            "submitted" => Some(Self::Submitted),
            "verified" => Some(Self::Verified),
            _ => None,
        }
    }
}

/// New event details.
#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct NewEvent {
    pub name: String,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    ApiComponent,
    JsonSchema,
    Clone,
)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Org))]
#[diesel(table_name = org_event)]
#[diesel(primary_key(event_id, org_id))]
pub struct OrgEvent {
    pub event_id: String,
    pub org_id: String,
    pub state: String,
}

/// Group details
#[derive(
    Debug,
    Clone,
    Serialize,
    Selectable,
    Deserialize,
    Queryable,
    Insertable,
    JsonSchema,
    ApiComponent,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = groups)]
pub struct Group {
    pub id: String,
    pub event_id: String,
    pub name: String,
    pub replacement: Option<String>,
    pub slug: String,
    pub num_rounds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct NewGroup {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct NewOrgSelfReg {
    pub name: String,
    pub event_id: Uuid,
    pub contact_email: String,
    pub contact_name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub enum Role {
    None,
    Admin,
    Org(Uuid),
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct SlimUser {
    pub email: String,
    pub role: Role,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            email: user.email,
            role: Role::Admin,
        }
    }
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    ApiComponent,
    JsonSchema,
    Clone,
)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Org))]
#[diesel(table_name = invite_queue)]
#[diesel(primary_key(event_id, org_id))]
pub struct InviteQueue {
    pub event_id: String,
    pub org_id: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    ApiComponent,
    JsonSchema,
    Clone,
)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Org))]
#[diesel(table_name = invite_queue)]
#[diesel(primary_key(event_id, org_id))]
pub struct InviteQueueEntry {
    pub event_id: String,
    pub org_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(belongs_to(Org))]
#[diesel(table_name = org_secrets)]
#[diesel(primary_key(org_id, secret))]
pub struct OrgSecret {
    pub org_id: String,
    pub secret: String,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = questionnaire)]
#[diesel(primary_key(id))]
pub struct Questionnaire {
    pub id: String,
    pub event_id: Option<String>,
    pub org_id: Option<String>,
    pub question_text: String,
    pub question_type: String,
    pub question_data: String,
    pub sort: i32,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
    AsChangeset,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = questionnaire_answers)]
#[diesel(belongs_to(Questionnaire))]
#[diesel(belongs_to(Org))]
#[diesel(belongs_to(Event))]
#[diesel(primary_key(question_id, event_id, org_id))]
pub struct QuestionnaireAnswer {
    pub question_id: String,
    pub event_id: String,
    pub org_id: String,
    pub question_answer: String,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
    AsChangeset,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = pairings)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(event, round, team_home, team_guest))]
pub struct Pairing {
    pub event: String,
    pub group_id: String,
    pub round: i32,
    pub table_num: i32,
    pub team_home: Option<String>,
    pub team_guest: Option<String>,
    pub points_home: Option<i32>,
    pub points_guest: Option<i32>,
    pub result: Option<String>,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
    AsChangeset,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = rooms)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(event, group_id, table_num_low, table_num_high))]
pub struct Room {
    pub event: String,
    pub group_id: String,
    pub table_num_low: i32,
    pub table_num_high: i32,
    pub room: String,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
    AsChangeset,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = results)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(event, round, team))]
pub struct TeamResult {
    pub event: String,
    pub group_id: String,
    pub round: i32,
    pub team: Option<String>,
    pub rank: Option<i32>,
    pub points_team: Option<i32>,
    pub points_player: Option<i32>,
    pub tie: Option<i32>,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
    Clone,
    AsChangeset,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = timetable)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(event, group_id, row_index))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TimetableEntry {
    pub event: String,
    pub group_id: String,
    pub row_index: i32,
    pub name: String,
    pub expected_time: NaiveDateTime,
    pub last_update: NaiveDateTime,
    pub state: String,
}
