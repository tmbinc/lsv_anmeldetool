use crate::models::TeamResult;
use diesel::prelude::*;
use uuid::Uuid;

use super::DbError;

pub fn set_results(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
    s_group: &Uuid,
    s_results: &[TeamResult],
) -> Result<(), DbError> {
    use crate::schema::results::dsl::*;

    diesel::delete(
        results
            .filter(event.eq(s_event.to_string()))
            .filter(group_id.eq(s_group.to_string())),
    )
    .execute(conn)?;

    diesel::insert_into(results)
        .values(s_results)
        .execute(conn)?;

    Ok(())
}

pub fn get_results(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
) -> Result<Vec<TeamResult>, DbError> {
    use crate::schema::results::dsl::*;

    Ok(results
        .select(TeamResult::as_select())
        .filter(event.eq(s_event.to_string()))
        .load(conn)?)
}
