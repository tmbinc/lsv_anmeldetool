use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find user by uid and return it.
pub fn find_org_by_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::Org>, DbError> {
    use crate::schema::orgs::dsl::*;

    let user = orgs
        .filter(id.eq(uid.to_string()))
        .first::<models::Org>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_org(
    conn: &mut SqliteConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<models::Org, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::orgs::dsl::*;

    let new_org = models::Org {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
        public: false,
    };

    diesel::insert_into(orgs).values(&new_org).execute(conn)?;

    Ok(new_org)
}
