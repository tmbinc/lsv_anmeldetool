use crate::actions::DbError;
use crate::models::{InviteQueue, InviteQueueEntry};
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

pub fn get_invites(conn: &mut SqliteConnection) -> Result<Vec<InviteQueueEntry>, DbError> {
    use crate::schema::invite_queue::dsl::*;
    Ok(invite_queue
        .select(InviteQueueEntry::as_select())
        .load::<InviteQueueEntry>(conn)?)
}
