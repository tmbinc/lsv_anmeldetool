use crate::actions::DbError;
use crate::models;
use diesel::prelude::*;
use uuid::Uuid;

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

pub fn update_org(
    conn: &mut SqliteConnection,
    uid: Uuid,
    data: models::Org,
) -> Result<models::Org, DbError> {
    use crate::schema::orgs::dsl::*;

    diesel::update(orgs.filter(id.eq(uid.to_string())))
        .set(&data)
        .execute(conn)?;
    Ok(data)
}

pub fn list_org(conn: &mut SqliteConnection) -> Result<Vec<models::Org>, DbError> {
    use crate::schema::orgs::dsl::*;

    let user = orgs.select(models::Org::as_select()).load(conn)?;

    Ok(user)
}

pub fn insert_new_org(conn: &mut SqliteConnection, nm: &str) -> Result<models::Org, DbError> {
    use crate::schema::orgs::dsl::*;

    let new_org = models::Org {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
        public: false,
        contact_email: None,
        contact_phone: None,
    };

    diesel::insert_into(orgs).values(&new_org).execute(conn)?;

    Ok(new_org)
}
