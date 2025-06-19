use crate::models::TimetableEntry;
use diesel::prelude::*;
use uuid::Uuid;

use super::DbError;

pub fn set_timetable(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
    s_timetable_entries: &[TimetableEntry],
) -> Result<(), DbError> {
    use crate::schema::timetable::dsl::*;

    diesel::delete(timetable.filter(event.eq(s_event.to_string()))).execute(conn)?;

    diesel::insert_into(timetable)
        .values(s_timetable_entries)
        .execute(conn)?;

    Ok(())
}

pub fn get_timetable(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
) -> Result<Vec<TimetableEntry>, DbError> {
    use crate::schema::timetable::dsl::*;

    Ok(timetable
        .select(TimetableEntry::as_select())
        .filter(event.eq(s_event.to_string()))
        .load(conn)?)
}
