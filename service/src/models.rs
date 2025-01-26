use crate::schema::{events, groups, org_event, orgs, teams};
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
    pub public: bool,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub confirmed_email: bool,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub enum EventOrgState {
    /// Org has not been included in this event.
    NotEnlisted,
    /// Org self-registered for this event.
    Registered,
    /// Org was confirmed (or manually added) to the event, has validated contact data.
    Created,
    /// Org has done any changes (i.e. clicked on final email link)
    Updated,
    /// Org has finalized entries for this event.
    Submitted,
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
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Submitted => "submitted",
        }
    }

    #[allow(unused)]
    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "not_enlisted" => Some(Self::NotEnlisted),
            "registered" => Some(Self::Registered),
            "created" => Some(Self::Created),
            "updated" => Some(Self::Updated),
            "submitted" => Some(Self::Submitted),
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
