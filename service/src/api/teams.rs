use crate::actions::teams::get_team_by_id;
use crate::actions::DbError;
use crate::errors::ErrorResponse;
use crate::models::{NewTeam, Team};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::api_operation;
use uuid::Uuid;

#[api_operation(summary = "get team list for an org + event")]
pub async fn get_org_teams(
    pool: web::Data<DbPool>,
    org_event_uid: Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Team>>, ErrorResponse> {
    let (org_uid, event_uid) = org_event_uid.into_inner();

    // TODO: check if org exists!
    // TODO: check if event exists!

    let event_org = web::block(move || -> Result<Option<Vec<Team>>, DbError> {
        let mut conn = pool.get()?;

        let teams = actions::teams::list_teams_by_org_event(&mut conn, org_uid, event_uid)?;

        Ok(Some(teams))
    })
    .await;

    let event_org = event_org?.map_err(error::ErrorInternalServerError)?;

    match event_org {
        Some(teams) => Ok(Json(teams)),
        None => Err(ErrorResponse::NotFound(format!(
            "No org found with UID: {org_uid}"
        ))),
    }
}

#[api_operation(summary = "update team")]
pub async fn update_team(
    pool: web::Data<DbPool>,
    team: Json<Team>,
) -> Result<Json<String>, ErrorResponse> {
    // TODO: check if org exists!
    // TODO: check if event exists!

    let event_org = web::block(move || -> Result<(), DbError> {
        let mut conn = pool.get()?;

        let team = team.into_inner();

        Ok(actions::teams::update_team(&mut conn, team)?)
    })
    .await;

    let _event_org = event_org?.map_err(error::ErrorInternalServerError)?;

    Ok(Json("ok".to_string()))
}

#[api_operation(summary = "create a new team")]
pub async fn add_team(
    pool: web::Data<DbPool>,
    team: Json<NewTeam>,
) -> Result<Json<Team>, ErrorResponse> {
    // TODO: check if org exists!
    // TODO: check if event exists!

    let event_org = web::block(move || -> Result<Team, DbError> {
        let mut conn = pool.get()?;

        let team = team.into_inner();
        Ok(actions::teams::create_team(&mut conn, team)?)
    })
    .await;

    let team = event_org?.map_err(error::ErrorInternalServerError)?;

    Ok(Json(team))
}

#[api_operation(summary = "delete a group")]
pub async fn delete_team(
    pool: web::Data<DbPool>,
    team_uid: Path<Uuid>,
) -> Result<Json<String>, ErrorResponse> {
    let team_uid = team_uid.into_inner();
    let group = web::block(move || -> Result<Option<usize>, DbError> {
        let mut conn = pool.get()?;

        let team = get_team_by_id(&mut conn, &team_uid)?;

        // FIXME: check team event_id is allowed by user

        if let Some(team) = team {
            Ok(Some(actions::teams::delete_team(&mut conn, team)?))
        } else {
            Ok(None)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match group {
        Some(1) => Ok(Json("ok".to_owned())),
        _ => Err(ErrorResponse::NotFound(format!(
            "No team found with UID: {team_uid}"
        ))),
    }
}

#[api_operation(summary = "get one team by ID")]
pub async fn get_team(
    pool: web::Data<DbPool>,
    team_uid: Path<Uuid>,
) -> Result<Json<Team>, ErrorResponse> {
    let team_uid = team_uid.into_inner();

    let team = web::block(move || {
        let mut conn = pool.get()?;

        actions::teams::get_team_by_id(&mut conn, &team_uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match team {
        // user was found; return 200 response with JSON formatted user object
        Some(team) => Ok(Json(team)),

        // user was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No team found with UID: {team_uid}"
        ))),
    }
}
