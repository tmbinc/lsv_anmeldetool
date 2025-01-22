use crate::actions::DbError;
use crate::models::{NewTeam, Team};
use crate::{actions, DbPool, ErrorResponse};
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

    let event_org = event_org?.map_err(error::ErrorInternalServerError)?;

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
