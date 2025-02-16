use crate::models::Pairing;
use diesel::prelude::*;
use uuid::Uuid;

use super::DbError;

pub fn set_pairings(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
    s_group: &Uuid,
    s_round: i32,
    s_pairings: &[Pairing],
) -> Result<(), DbError> {
    use crate::schema::pairings::dsl::*;

    diesel::delete(
        pairings
            .filter(event.eq(s_event.to_string()))
            .filter(group_id.eq(s_group.to_string()))
            .filter(round.eq(s_round)),
    )
    .execute(conn)?;

    diesel::insert_into(pairings)
        .values(s_pairings)
        .execute(conn)?;

    Ok(())
}

pub fn get_pairings(conn: &mut SqliteConnection, s_event: &Uuid) -> Result<Vec<Pairing>, DbError> {
    use crate::schema::pairings::dsl::*;

    Ok(pairings
        .select(Pairing::as_select())
        .filter(event.eq(s_event.to_string()))
        .load(conn)?)
}
