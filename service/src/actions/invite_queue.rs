use crate::actions::DbError;
use crate::models::InviteQueue;
use crate::schema;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_invite(
    conn: &mut SqliteConnection,
    event_id: &Uuid,
    org_id: &Uuid,
) -> Result<(), DbError> {
    let invite = InviteQueue {
        event_id: event_id.to_string(),
        org_id: org_id.to_string(),
        created_at: None,
    };
    diesel::insert_into(schema::invite_queue::table)
        .values(&invite)
        .execute(conn)?;
    Ok(())
}
