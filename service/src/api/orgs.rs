use crate::actions::DbError;
use crate::models::{self, Event, EventOrgState, Org, Team};
use crate::{actions, DbPool, ErrorResponse};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

// Org-specific information for an event
#[derive(Debug, Clone, Serialize, ApiComponent, JsonSchema)]
pub struct OrgEvent {
    /// Event details
    pub event: Event,
    /// org state for this event
    pub state: EventOrgState,
    /// teams for this event
    pub teams: Vec<Team>,
}

/// Finds org by UID.
#[api_operation(summary = "get one org by ID")]
pub async fn get_org(
    pool: web::Data<DbPool>,
    user_uid: Path<Uuid>,
) -> Result<Json<Org>, ErrorResponse> {
    let user_uid = user_uid.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::orgs::find_org_by_uid(&mut conn, user_uid)
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    match user {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => Ok(Json(user)),

        // user was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No user found with UID: {user_uid}"
        ))),
    }
}

#[api_operation(summary = "update org")]
pub async fn update_org(
    pool: web::Data<DbPool>,
    user_uid: Path<Uuid>,
    data: Json<Org>,
) -> Result<Json<Org>, ErrorResponse> {
    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::orgs::update_org(&mut conn, *user_uid, data.into_inner())
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    Ok(Json(user))
}

/// List orgs
#[api_operation(summary = "get list of orgs")]
pub async fn get_orgs(pool: web::Data<DbPool>) -> Result<Json<Vec<Org>>, ErrorResponse> {
    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::orgs::list_org(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add an org")]
pub async fn add_org(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewOrg>,
) -> Result<Json<Org>, ErrorResponse> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let org = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::orgs::insert_new_org(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(Json(org))
}

#[api_operation(summary = "get event list for a org")]
pub async fn get_org_events(
    pool: web::Data<DbPool>,
    org_uid: Path<Uuid>,
) -> Result<Json<Vec<OrgEvent>>, ErrorResponse> {
    let org_uid = org_uid.into_inner();

    let event_org = web::block(move || -> Result<Option<Vec<OrgEvent>>, DbError> {
        let mut conn = pool.get()?;

        let events = actions::org_event::list_org_events(&mut conn, org_uid)?;

        Ok(match events {
            Some(events) => Some(
                events
                    .into_iter()
                    .map(|event| OrgEvent {
                        event: event,
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
        None => Err(ErrorResponse::NotFound(format!(
            "No org found with UID: {org_uid}"
        ))),
    }
}
