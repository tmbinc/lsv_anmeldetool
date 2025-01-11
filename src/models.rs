use crate::schema::orgs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = orgs)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub public: bool,
}

/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
