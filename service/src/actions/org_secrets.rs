use crate::actions::DbError;
use crate::models::OrgSecret;
use crate::schema::org_secrets;
use actix_web::cookie::Key;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_org_secret(conn: &mut SqliteConnection, org_uid: &Uuid) -> Result<(), DbError> {
    let key = Key::generate();
    let secret_key = &key.master()[0..15]; // 15 bytes are sufficiently long and is divisible by 3 (for base64 encoding)
    let secret = URL_SAFE.encode(secret_key);

    let secret = OrgSecret {
        org_id: org_uid.to_string(),
        secret: secret,
    };
    diesel::insert_into(org_secrets::table)
        .values(&secret)
        .on_conflict(org_secrets::all_columns)
        .do_nothing()
        .execute(conn)?;
    Ok(())
}

pub fn get_org_secret(
    conn: &mut SqliteConnection,
    org_uid: &Uuid,
) -> Result<Option<String>, DbError> {
    use crate::schema::org_secrets::org_id;

    let secret = org_secrets::table
        .filter(org_id.eq_all(org_uid.to_string()))
        .first::<OrgSecret>(conn)
        .optional()?;

    Ok(secret.map(|secret| secret.secret))
}

pub fn check_org_auth_token(
    conn: &mut SqliteConnection,
    org_uid: Uuid,
    token: &str,
) -> Result<bool, DbError> {
    use crate::schema::org_secrets::org_id;

    let items = org_secrets::table
        .filter(org_id.eq_all(org_uid.to_string()))
        .load::<OrgSecret>(conn)?;

    for item in items {
        if sha256::digest(item.secret) == sha256::digest(token) {
            return Ok(true);
        }
    }

    Ok(false)
}
