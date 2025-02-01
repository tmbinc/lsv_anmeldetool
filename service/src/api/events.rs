use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{self, Event, EventOrgState, EventOrgStateUpdate, NewEvent, Org, Role, Team};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

// Event-specific information for an org
#[derive(Debug, Clone, Serialize, ApiComponent, JsonSchema)]
pub struct EventOrg {
    /// Org details
    pub org: Org,
    /// org state for this event
    pub state: EventOrgState,
    /// teams for this event
    pub teams: Vec<Team>,
}

/// Get event details
#[api_operation(summary = "get one event by ID", skip_args = "user")]
pub async fn get_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<Event>, ErrorResponse> {
    match user.role {
        Role::Admin | Role::None | Role::Org(_) => {}
    };

    let event_uid = event_uid.into_inner();

    let event = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::find_event_by_uid(&mut conn, &event_uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match event {
        Some(event) => Ok(Json(event)),

        // event was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No event found with UID: {event_uid}"
        ))),
    }
}

#[api_operation(summary = "update event", skip_args = "user")]
pub async fn update_event(
    pool: web::Data<DbPool>,
    event_uid: Path<Uuid>,
    user: LoggedUser,
    data: Json<Event>,
) -> Result<Json<Event>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::update_event(&mut conn, *event_uid, data.into_inner())
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    Ok(Json(event))
}

/// List events
#[api_operation(summary = "get list of events", skip_args = "user")]
pub async fn get_events(
    pool: web::Data<DbPool>,
    user: LoggedUser,
) -> Result<Json<Vec<Event>>, ErrorResponse> {
    let only_public = match user.role {
        Role::Admin => false,
        _ => true,
    };

    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::list_events(&mut conn, only_public)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add an event", skip_args = "user")]
pub async fn add_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    form: web::Json<NewEvent>,
) -> Result<Json<Event>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let org = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::insert_new_event(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;
    Ok(Json(org))
}

#[api_operation(summary = "get org list for a given event", skip_args = "user")]
pub async fn get_event_orgs(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<Vec<EventOrg>>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_uid = event_uid.into_inner();

    let event_org = web::block(move || -> Result<Option<Vec<EventOrg>>, DbError> {
        let mut conn = pool.get()?;

        let orgs = actions::org_event::list_event_orgs(&mut conn, event_uid)?;

        Ok(match orgs {
            Some(orgs) => Some(
                orgs.into_iter()
                    .map(|(org, state)| EventOrg {
                        org,
                        state,
                        teams: [].into(),
                    })
                    .collect(),
            ),
            None => None,
        })
    })
    .await;

    let event_org = event_org?.map_err(error::ErrorInternalServerError)?;

    match event_org {
        Some(event_org) => Ok(Json(event_org)),

        // event was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No event found with UID: {event_uid}"
        ))),
    }
}

#[api_operation(summary = "set status for event per org", skip_args = "user")]
pub async fn set_event_org_state(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_org_uid: Path<(Uuid, Uuid)>,
    state_update: Json<EventOrgStateUpdate>,
) -> Result<Json<String>, ErrorResponse> {
    let (event_uid, org_uid) = event_org_uid.into_inner();

    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_org = web::block(move || {
        let mut conn = pool.get()?;

        let state_update = state_update.into_inner();
        let org_event = models::OrgEvent {
            event_id: event_uid.to_string(),
            org_id: org_uid.to_string(),
            state: state_update.state.to_db().to_string(),
        };

        // TODO: check that event and org actually exists
        match state_update.state {
            EventOrgState::NotEnlisted => {
                actions::org_event::delete_event_org(&mut conn, org_event)
            }
            _ => actions::org_event::update_event_org_state(&mut conn, org_event),
        }
    })
    .await;

    let _event_org = event_org?.map_err(error::ErrorInternalServerError)?;

    Ok(Json("ok".to_string()))
}
