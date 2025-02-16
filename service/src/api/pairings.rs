use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::Room;
use crate::models::{Group, Pairing, Role};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SetPairingPayload {
    pairings: Vec<Pairing>,
}

#[api_operation(summary = "set pairings for a given event + group", skip_args = "user")]
pub async fn set_pairings(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<(Uuid, Uuid, i32)>,
    data: Json<SetPairingPayload>,
) -> Result<Json<()>, ErrorResponse> {
    let mut data = data.into_inner().pairings;
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

        actions::pairings::set_pairings(&mut conn, &event_uid, &group_uid, round, &data)?;
        Ok(())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct PairingEntry {
    team_home: Option<String>,
    team_home_org: Option<String>,
    points_home: Option<i32>,
    team_guest: Option<String>,
    team_guest_org: Option<String>,
    points_guest: Option<i32>,
    table: i32,
    group: String,
    round: i32,
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct PairingForEvent {
    event_name: String,
    pairings: Vec<PairingEntry>,
    groups: Vec<Group>,
    rooms: Vec<Room>,
}

#[api_operation(summary = "get pairings for a given event + group", skip_args = "user")]
pub async fn get_pairings(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<PairingForEvent>, ErrorResponse> {
    let _ = user;
    let event_uid = event_uid.into_inner();
    let data = web::block(move || -> Result<Option<PairingForEvent>, DbError> {
        let mut conn = pool.get()?;

        let Some(event) = actions::events::find_event_by_uid(&mut conn, &event_uid)? else {
            return Ok(None);
        };

        let pairing = actions::pairings::get_pairings(&mut conn, &event_uid)?;
        let groups = actions::groups::list_groups(&mut conn, &event_uid)?;
        let Some(orgs) = actions::org_event::list_event_orgs(&mut conn, &event_uid)? else {
            return Ok(None);
        };

        let rooms = actions::rooms::get_rooms(&mut conn, &event_uid)?;

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

        let mut pairings = Vec::new();

        for p in pairing {
            let team_home = if let Some(team_home) = p.team_home {
                teams_map.get(&team_home)
            } else {
                None
            };
            let team_guest = if let Some(team_guest) = p.team_guest {
                teams_map.get(&team_guest)
            } else {
                None
            };
            pairings.push(PairingEntry {
                team_home: team_home.map(|(_, team)| team.name.clone()),
                team_home_org: team_home.map(|(or, _)| or.name.clone()),
                points_home: p.points_home,
                team_guest: team_guest.map(|(_, team)| team.name.clone()),
                team_guest_org: team_guest.map(|(or, _)| or.name.clone()),
                points_guest: p.points_guest,
                table: p.table_num,
                group: p.group_id,
                round: p.round,
            });
        }

        Ok(Some(PairingForEvent {
            event_name: event.name,
            pairings,
            groups,
            rooms,
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
