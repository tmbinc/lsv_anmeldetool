use crate::models::Room;
use diesel::prelude::*;
use uuid::Uuid;

use super::DbError;

pub fn set_rooms(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
    s_group: &Uuid,
    s_rooms: &[Room],
) -> Result<(), DbError> {
    use crate::schema::rooms::dsl::*;

    diesel::delete(
        rooms
            .filter(event.eq(s_event.to_string()))
            .filter(group_id.eq(s_group.to_string())),
    )
    .execute(conn)?;

    diesel::insert_into(rooms).values(s_rooms).execute(conn)?;

    Ok(())
}

pub fn get_rooms(conn: &mut SqliteConnection, s_event: &Uuid) -> Result<Vec<Room>, DbError> {
    use crate::schema::rooms::dsl::*;

    Ok(rooms
        .select(Room::as_select())
        .filter(event.eq(s_event.to_string()))
        .load(conn)?)
}
