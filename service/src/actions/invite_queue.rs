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

pub fn get_one_invite(conn: &mut SqliteConnection) -> Result<Option<InviteQueueEntry>, DbError> {
    use crate::schema::invite_queue::dsl::*;
    Ok(invite_queue
        .select(InviteQueueEntry::as_select())
        .first::<InviteQueueEntry>(conn)
        .optional()?)
}

pub fn delete_invite(
    conn: &mut SqliteConnection,
    org_uid: &Uuid,
    event_uid: &Uuid,
) -> Result<usize, DbError> {
    use crate::schema::invite_queue::dsl::*;
    Ok(diesel::delete(
        invite_queue
            .filter(event_id.eq(event_uid.to_string()))
            .filter(org_id.eq(org_uid.to_string())),
    )
    .execute(conn)?)
}
