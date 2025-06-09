use crate::actions::teams::get_team_by_id;
use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{NewTeam, Role, Team};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

#[api_operation(summary = "get team list for an org + event", skip_args = "user")]
pub async fn get_org_teams(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_event_uid: Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Team>>, ErrorResponse> {
    let (org_uid, event_uid) = org_event_uid.into_inner();

    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    // TODO: check if org exists!
    // TODO: check if event exists!

    let event_org = web::block(move || -> Result<Option<Vec<Team>>, DbError> {
        let mut conn = pool.get()?;

        let teams = actions::teams::list_teams_by_org_event(&mut conn, &org_uid, &event_uid)?;

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

#[api_operation(summary = "update team", skip_args = "user")]
pub async fn update_team(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    team: Json<Team>,
) -> Result<Json<String>, ErrorResponse> {
    // TODO: check if org exists!

    let is_user_edit = match user.role {
        Role::Admin => false,
        Role::Org(org) if org.to_string() == team.org => true,
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let success = web::block(move || -> Result<bool, DbError> {
        let mut conn = pool.get()?;

        let team = team.into_inner();

        if is_user_edit {
            // For user edits, verify a few additional things:

            if let Some(old_team) =
                actions::teams::get_team_by_id(&mut conn, &Uuid::parse_str(&team.id)?)?
            {
                // Must not move teams across events or orgs
                if old_team.event != team.event || old_team.org != team.org {
                    return Ok(false);
                }
            } else {
                // team does not exist
                return Ok(false);
            }

            if let Some(event) =
                actions::events::find_event_by_uid(&mut conn, &Uuid::parse_str(&team.event)?)?
            {
                if !event.allow_user_changes {
                    return Ok(false);
                }
            } else {
                // event does not exist (anymore?)
                return Ok(false);
            }
        }

        actions::teams::update_team(&mut conn, team)?;
        Ok(true)
    })
    .await;

    let success = success?.map_err(error::ErrorInternalServerError)?;

    if success {
        Ok(Json("ok".to_string()))
    } else {
        return Err(ErrorResponse::Unauthorized("".to_string()));
    }
}

#[api_operation(summary = "create a new team", skip_args = "user")]
pub async fn add_team(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    team: Json<NewTeam>,
) -> Result<Json<Team>, ErrorResponse> {
    // TODO: check if org exists!
    // TODO: check if event exists!

    let is_user_edit = match user.role {
        Role::Admin => false,
        Role::Org(org) if org.to_string() == team.org => true,
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_org = web::block(move || -> Result<Option<Team>, DbError> {
        let mut conn = pool.get()?;

        let team = team.into_inner();

        if is_user_edit {
            if let Some(event) =
                actions::events::find_event_by_uid(&mut conn, &Uuid::parse_str(&team.event)?)?
            {
                if !event.allow_user_changes {
                    return Ok(None);
                }
            } else {
                // event does not exist (anymore?)
                return Ok(None);
            }
        }

        Ok(Some(actions::teams::create_team(&mut conn, team)?))
    })
    .await;

    if let Some(team) = event_org?.map_err(error::ErrorInternalServerError)? {
        Ok(Json(team))
    } else {
        return Err(ErrorResponse::Unauthorized("".to_string()));
    }
}

#[api_operation(summary = "delete a team", skip_args = "user")]
pub async fn delete_team(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    team_uid: Path<Uuid>,
) -> Result<Json<String>, ErrorResponse> {
    let team_uid = team_uid.into_inner();

    let group = web::block(move || -> Result<Option<usize>, DbError> {
        let mut conn = pool.get()?;

        let team = get_team_by_id(&mut conn, &team_uid)?;

        if let Some(team) = team {
            let is_user_edit = match user.role {
                Role::Admin => false,
                Role::Org(org) => {
                    if org.to_string() != team.org {
                        return Ok(None);
                    } else {
                        true
                    }
                }
                _ => return Ok(None),
            };

            if is_user_edit {
                if let Some(event) =
                    actions::events::find_event_by_uid(&mut conn, &Uuid::parse_str(&team.event)?)?
                {
                    if !event.allow_user_changes {
                        return Ok(None);
                    }
                } else {
                    // event does not exist (anymore?)
                    return Ok(None);
                }
            }
            Ok(Some(actions::teams::delete_team(&mut conn, team)?))
        } else {
            Ok(None)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match group {
        None => Err(ErrorResponse::Unauthorized("".to_string())),
        Some(1) => Ok(Json("ok".to_owned())),
        _ => Err(ErrorResponse::NotFound(format!(
            "No team found with UID: {team_uid}"
        ))),
    }
}

#[api_operation(summary = "get one team by ID", skip_args = "user")]
pub async fn get_team(
    pool: web::Data<DbPool>,
    user: LoggedUser,
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
        Some(team) => {
            let authorized = match user.role {
                Role::Admin => true,
                Role::Org(org) if org.to_string() == team.org => true,
                _ => false,
            };

            if authorized {
                Ok(Json(team))
            } else {
                Err(ErrorResponse::Unauthorized("".to_string()))
            }
        }

        // user was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No team found with UID: {team_uid}"
        ))),
    }
}

#[derive(Deserialize, ApiComponent, JsonSchema)]
pub struct TeamReady {
    team_id: Uuid,
    ready: bool,
}

#[api_operation(summary = "update team readiness", skip_args = "user")]
pub async fn set_team_present(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    team_ready: Json<TeamReady>,
) -> Result<Json<String>, ErrorResponse> {
    let team_ready = team_ready.into_inner();

    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_org = web::block(move || -> Result<(), DbError> {
        let mut conn = pool.get()?;

        Ok(actions::teams::set_team_present(
            &mut conn,
            &team_ready.team_id,
            team_ready.ready,
        )?)
    })
    .await;

    let _event_org = event_org?.map_err(error::ErrorInternalServerError)?;

    Ok(Json("ok".to_string()))
}
