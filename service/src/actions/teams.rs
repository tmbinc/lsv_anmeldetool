use crate::actions::DbError;
use crate::models::{NewTeam, Team};
use crate::schema;
use diesel::prelude::*;
use uuid::Uuid;

pub fn list_teams_by_org_event(
    conn: &mut SqliteConnection,
    org_uid: &Uuid,
    event_uid: &Uuid,
) -> Result<Vec<Team>, DbError> {
    use crate::schema::teams::dsl::*;

    let user = teams
        .select(Team::as_select())
        .filter(event.eq(event_uid.to_string()))
        .filter(org.eq(org_uid.to_string()))
        .load(conn)?;

    Ok(user)
}

pub fn update_team(conn: &mut SqliteConnection, team: Team) -> Result<(), DbError> {
    diesel::insert_into(schema::teams::table)
        .values(&team)
        .on_conflict(schema::teams::id)
        .do_update()
        .set(&team)
        .execute(conn)?;
    Ok(())
}

pub fn create_team(conn: &mut SqliteConnection, team: NewTeam) -> Result<Team, DbError> {
    let team = Team {
        id: Uuid::new_v4().to_string(),
        name: team.name,
        event: team.event,
        org: team.org,
        group_id: None,
        contact_name: None,
        contact_phone: None,
        present: false,
    };
    diesel::insert_into(schema::teams::table)
        .values(&team)
        .execute(conn)?;
    Ok(team)
}

pub fn delete_team(conn: &mut SqliteConnection, team: Team) -> Result<usize, DbError> {
    let res = diesel::delete(&team).execute(conn)?;

    Ok(res)
}

pub fn get_team_by_id(
    conn: &mut SqliteConnection,
    team_uid: &Uuid,
) -> Result<Option<Team>, DbError> {
    use crate::schema::teams::dsl::*;

    Ok(teams
        .filter(id.eq(team_uid.to_string()))
        .first::<Team>(conn)
        .optional()?)
}

pub fn set_team_present(
    conn: &mut SqliteConnection,
    team_id: &Uuid,
    ready: bool,
) -> Result<(), DbError> {
    diesel::update(schema::teams::table)
        .filter(schema::teams::id.eq(team_id.to_string()))
        .set(schema::teams::present.eq(ready))
        .execute(conn)?;
    Ok(())
}
