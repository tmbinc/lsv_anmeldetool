use crate::actions::DbError;
use crate::models::Org;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub fn find_org_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> Result<Option<Org>, DbError> {
    use crate::schema::orgs::dsl::*;

    let user = orgs
        .filter(id.eq(uid.to_string()))
        .first::<Org>(conn)
        .optional()?;

    Ok(user)
}

pub fn update_org(conn: &mut SqliteConnection, uid: Uuid, data: &Org) -> Result<(), DbError> {
    use crate::schema::orgs::dsl::*;

    diesel::update(orgs.filter(id.eq(uid.to_string())))
        .set(data)
        .execute(conn)?;
    Ok(())
}

pub fn list_org(conn: &mut SqliteConnection) -> Result<Vec<Org>, DbError> {
    use crate::schema::orgs::dsl::*;

    let user = orgs.select(Org::as_select()).load(conn)?;

    Ok(user)
}

pub fn insert_new_org(conn: &mut SqliteConnection, nm: &str) -> Result<Org, DbError> {
    use crate::schema::orgs::dsl::*;

    let new_org = Org {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
        name_additional: None,
        genus: None,
        public: false,
        contact_email: None,
        contact_phone: None,
        contact_name: None,
        contact_email_pending: None,
        last_update: Utc::now().naive_utc(),
    };

    diesel::insert_into(orgs).values(&new_org).execute(conn)?;

    Ok(new_org)
}
