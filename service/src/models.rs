use crate::schema::{events, org_event, orgs, teams};
use apistos::ApiComponent;
use chrono::NaiveDate;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub contact_phone: Option<String>,
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
)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: String,
    pub event: String,
    pub org: String,
    pub name: String,
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
    pub begin: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub enum EventOrgState {
    NotEnlisted,
    Created,
    Updated,
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
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Submitted => "submitted",
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "not_enlisted" => Some(Self::NotEnlisted),
            "created" => Some(Self::Created),
            "updated" => Some(Self::Updated),
            "submitted" => Some(Self::Submitted),
            _ => None,
        }
    }
}

// Event-specific information for an org
#[derive(Debug, Clone, Serialize, ApiComponent, JsonSchema)]
pub struct EventOrg {
    /// Org details
    pub org: Org,
    /// org state for this event
    pub state: EventOrgState,
    /// teams for this event
    pub teams: Vec<Team>,
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
