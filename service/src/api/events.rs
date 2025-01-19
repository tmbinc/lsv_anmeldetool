use crate::actions::DbError;
use crate::models::{self, Event, EventOrgState, EventOrgStateUpdate, NewEvent, Org, Team};
use crate::{actions, DbPool, ErrorResponse};
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
#[api_operation(summary = "get one event by ID")]
pub async fn get_event(
    pool: web::Data<DbPool>,
    event_uid: Path<Uuid>,
) -> Result<Json<Event>, ErrorResponse> {
    let event_uid = event_uid.into_inner();

    let event = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::find_event_by_uid(&mut conn, event_uid)
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    match event {
        Some(event) => Ok(Json(event)),

        // event was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No event found with UID: {event_uid}"
        ))),
    }
}

#[api_operation(summary = "update event")]
pub async fn update_event(
    pool: web::Data<DbPool>,
    event_uid: Path<Uuid>,
    data: Json<Event>,
) -> Result<Json<Event>, ErrorResponse> {
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
#[api_operation(summary = "get list of events")]
pub async fn get_events(pool: web::Data<DbPool>) -> Result<Json<Vec<Event>>, ErrorResponse> {
    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::list_events(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add an event")]
pub async fn add_event(
    pool: web::Data<DbPool>,
    form: web::Json<NewEvent>,
) -> Result<Json<Event>, ErrorResponse> {
    let org = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::insert_new_event(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;
    Ok(Json(org))
}

#[api_operation(summary = "get org list for a given event")]
pub async fn get_event_orgs(
    pool: web::Data<DbPool>,
    event_uid: Path<Uuid>,
) -> Result<Json<Vec<EventOrg>>, ErrorResponse> {
    let event_uid = event_uid.into_inner();

    let event_org = web::block(move || -> Result<Option<Vec<EventOrg>>, DbError> {
        let mut conn = pool.get()?;

        let orgs = actions::org_event::list_event_orgs(&mut conn, event_uid)?;

        Ok(match orgs {
            Some(orgs) => Some(
                orgs.into_iter()
                    .map(|org| EventOrg {
                        org: org,
                        state: EventOrgState::Created,
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

#[api_operation(summary = "set status for event per org")]
pub async fn set_event_org_state(
    pool: web::Data<DbPool>,
    event_org_uid: Path<(Uuid, Uuid)>,
    state_update: Json<EventOrgStateUpdate>,
) -> Result<Json<String>, ErrorResponse> {
    let (event_uid, org_uid) = event_org_uid.into_inner();
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
