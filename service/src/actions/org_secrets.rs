use crate::actions::DbError;
use crate::errors::ErrorResponse;
use crate::models::OrgSecret;
use crate::schema::org_secrets;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_org_secret(conn: &mut SqliteConnection, org_uid: Uuid) -> Result<(), DbError> {
    let secret = OrgSecret {
        org_id: org_uid.to_string(),
        secret: "HelloSecretString".into(),
    };
    diesel::insert_into(org_secrets::table)
        .values(&secret)
        .execute(conn)?;
    Ok(())
}

pub fn check_org_auth_token(
    conn: &mut SqliteConnection,
    org_uid: Uuid,
    token: &str,
) -> Result<bool, DbError> {
    use crate::schema::org_secrets::org_id;

    let mut items = org_secrets::table
        .filter(org_id.eq_all(org_uid.to_string()))
        .load::<OrgSecret>(conn)?;

    for item in items {
        if sha256::digest(item.secret) == sha256::digest(token) {
            return Ok(true);
        }
    }

    Ok(false)
}
