use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{Group, Role, TeamResult};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SetResultsPayload {
    results: Vec<TeamResult>,
}

#[api_operation(summary = "set results for a given event + group", skip_args = "user")]
pub async fn set_results(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<(Uuid, Uuid, i32)>,
    data: Json<SetResultsPayload>,
) -> Result<Json<()>, ErrorResponse> {
    let mut data = data.into_inner().results;
    let (event_uid, group_uid, round) = path.into_inner();

    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let data = web::block(move || -> Result<(), DbError> {
        let mut conn = pool.get()?;

        for p in data.iter_mut() {
            p.event = event_uid.to_string();
            p.group_id = group_uid.to_string();
            p.round = round;
        }

        actions::results::set_results(&mut conn, &event_uid, &group_uid, &data)?;
        Ok(())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct ResultEntry {
    team: Option<String>,
    team_org: Option<String>,
    rank: Option<i32>,
    points_team: Option<i32>,
    points_player: Option<i32>,
    tie: Option<i32>,
    group: String,
    round: i32,
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct ResultsForEvent {
    event_name: String,
    results: Vec<ResultEntry>,
    groups: Vec<Group>,
}

#[api_operation(summary = "get results for a given event + group", skip_args = "user")]
pub async fn get_results(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<ResultsForEvent>, ErrorResponse> {
    let is_admin = match user.role {
        Role::Admin => true,
        _ => false,
    };

    let event_uid = event_uid.into_inner();
    let data = web::block(move || -> Result<Option<ResultsForEvent>, DbError> {
        let mut conn = pool.get()?;

        let Some(event) = actions::events::find_event_by_uid(&mut conn, &event_uid)? else {
            return Ok(None);
        };

        // Refuse to list results if the user is not admin and the results aren't marked public yet.
        if !is_admin && !event.results_are_public {
            return Ok(None);
        }

        let results = actions::results::get_results(&mut conn, &event_uid)?;
        let groups = actions::groups::list_groups(&mut conn, &event_uid)?;
        let Some(orgs) = actions::org_event::list_event_orgs(&mut conn, &event_uid)? else {
            return Ok(None);
        };

        let mut teams_map = HashMap::new();

        for (this_org, _) in orgs {
            let teams = actions::teams::list_teams_by_org_event(
                &mut conn,
                &Uuid::parse_str(&this_org.id).unwrap(),
                &event_uid,
            )?;

            for t in teams.into_iter() {
                let id = t.id.clone();
                teams_map.insert(id, (this_org.clone(), t));
            }
        }

        let mut results_entries = Vec::new();

        for p in results {
            let team = if let Some(team) = p.team {
                teams_map.get(&team)
            } else {
                None
            };

            results_entries.push(ResultEntry {
                team: team.map(|(_, team)| team.name.clone()),
                team_org: team.map(|(org, _)| org.name.clone()),
                rank: p.rank,
                points_team: p.points_team,
                points_player: p.points_player,
                tie: p.tie,
                group: p.group_id,
                round: p.round,
            });
        }

        Ok(Some(ResultsForEvent {
            event_name: event.name,
            results: results_entries,
            groups,
        }))
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    match data {
        Some(data) => Ok(Json(data)),
        None => Err(ErrorResponse::NotFound(format!("Event not found"))),
    }
}
