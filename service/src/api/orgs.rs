use crate::actions::events::find_event_by_uid;
use crate::actions::invite_queue::create_invite;
use crate::actions::org_event::{get_event_org_state, update_event_org_state};
use crate::actions::org_secrets::create_org_secret;
use crate::actions::org_secrets::get_org_secret;
use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{self, Event, EventOrgState, NewOrgSelfReg, Org, Role, Team};
use crate::{actions, DbPool};
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
#[api_operation(summary = "get one org by ID", skip_args = "user")]
pub async fn get_org(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_id: Path<Uuid>,
) -> Result<Json<Org>, ErrorResponse> {
    let org_id = org_id.into_inner();

    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_id => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let org = web::block(move || {
        let mut conn = pool.get()?;

        actions::orgs::find_org_by_uid(&mut conn, &org_id)
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    match org {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => Ok(Json(user)),

        // user was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No user found with UID: {org_id}"
        ))),
    }
}

#[api_operation(summary = "update org", skip_args = "user")]
pub async fn update_org(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_uid: Path<Uuid>,
    data: Json<Org>,
) -> Result<Json<Org>, ErrorResponse> {
    let data = data.into_inner();
    let org_uid = org_uid.into_inner();

    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let data = web::block(move || -> Result<Org, DbError> {
        let mut conn = pool.get()?;

        actions::orgs::update_org(&mut conn, org_uid, &data)?;
        Ok(data)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}

/// List orgs
#[api_operation(summary = "get list of orgs", skip_args = "user")]
pub async fn get_orgs(
    pool: web::Data<DbPool>,
    user: LoggedUser,
) -> Result<Json<Vec<Org>>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::orgs::list_org(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add an org", skip_args = "user")]
pub async fn add_org(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    form: web::Json<models::NewOrg>,
) -> Result<Json<Org>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

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

#[api_operation(summary = "get event list for a org", skip_args = "user")]
pub async fn get_org_events(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_uid: Path<Uuid>,
) -> Result<Json<Vec<OrgEvent>>, ErrorResponse> {
    let org_uid = org_uid.into_inner();

    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_org = web::block(move || -> Result<Option<Vec<OrgEvent>>, DbError> {
        let mut conn = pool.get()?;

        let events = actions::org_event::list_org_events(&mut conn, org_uid)?;

        Ok(match events {
            Some(events) => Some(
                events
                    .into_iter()
                    .map(|(event, state)| OrgEvent {
                        event,
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
        None => Err(ErrorResponse::NotFound(format!(
            "No org found with UID: {org_uid}"
        ))),
    }
}

#[api_operation(summary = "get event status for an org", skip_args = "user")]
pub async fn get_org_event_state(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_event_uid: Path<(Uuid, Uuid)>,
) -> Result<Json<EventOrgState>, ErrorResponse> {
    let (org_uid, event_uid) = org_event_uid.into_inner();

    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_org = web::block(move || -> Result<Option<EventOrgState>, DbError> {
        let mut conn = pool.get()?;

        Ok(actions::org_event::get_event_org_state(
            &mut conn, &event_uid, &org_uid,
        )?)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match event_org {
        Some(event_org) => Ok(Json(event_org)),
        None => Err(ErrorResponse::NotFound(format!(
            "No event/org found with UID: {org_uid}/{event_uid}"
        ))),
    }
}

#[api_operation(summary = "self-register a new org", skip_args = "user")]
pub async fn self_register_org(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    data: Json<NewOrgSelfReg>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin | Role::None | Role::Org(_) => {}
    };

    let data = data.into_inner();

    let success = web::block(move || -> Result<bool, DbError> {
        let mut conn = pool.get()?;

        let event = find_event_by_uid(&mut conn, &data.event_id)?;

        match event {
            Some(event) => {
                if event.public {
                    let new_org = actions::orgs::insert_new_org(&mut conn, &data.name)?;

                    let new_org = Org {
                        name: data.name,
                        public: false,
                        contact_email: Some(data.contact_email),
                        contact_name: Some(data.contact_name),
                        genus: Some("f".into()),
                        ..new_org
                    };

                    actions::orgs::update_org(
                        &mut conn,
                        Uuid::parse_str(&new_org.id).expect("generated a uuid that didn't work"),
                        &new_org,
                    )?;

                    actions::org_event::update_event_org_state(
                        &mut conn,
                        models::OrgEvent {
                            event_id: event.id,
                            org_id: new_org.id,
                            state: EventOrgState::Registered.to_db().into(),
                        },
                    )?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if success {
        Ok(Json("success".to_string()))
    } else {
        Err(ErrorResponse::Unauthorized(
            "No valid event for self-registration".into(),
        ))
    }
}

#[api_operation(summary = "invite an org to an event", skip_args = "user")]
pub async fn invite_org_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<(Uuid, Uuid)>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let (org_uid, event_uid) = path.into_inner();

    let success = web::block(move || -> Result<bool, DbError> {
        let mut conn = pool.get()?;

        match get_event_org_state(&mut conn, &event_uid, &org_uid)? {
            Some(EventOrgState::Registered) => {
                // Create org secret
                create_org_secret(&mut conn, &org_uid).expect("failed to create org secret");

                // Add invitation to queue
                create_invite(&mut conn, &event_uid, &org_uid).expect("failed to create invite");

                // Update state to "invited"
                update_event_org_state(
                    &mut conn,
                    models::OrgEvent {
                        event_id: event_uid.to_string(),
                        org_id: org_uid.to_string(),
                        state: EventOrgState::Invited.to_db().into(),
                    },
                )
                .expect("failed to update org event state");

                Ok(true)
            }
            // If state is not "registered", then we can't invite.
            _ => Ok(false),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if success {
        Ok(Json("invite created".into()))
    } else {
        Err(ErrorResponse::NotFound(
            "Org not in registered state for event".into(),
        ))
    }
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct Invite {
    pub event: Event,
    pub org: Org,
    pub secret: String,
}

#[api_operation(summary = "get invites", skip_args = "user")]
pub async fn list_invites(
    pool: web::Data<DbPool>,
    user: LoggedUser,
) -> Result<Json<Vec<Invite>>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let success = web::block(move || -> Result<Vec<Invite>, DbError> {
        let mut conn = pool.get()?;
        let invite_queue_entries = actions::invite_queue::get_invites(&mut conn)?;

        let invite_queue_entries = invite_queue_entries
            .into_iter()
            .filter_map(|entry| {
                let org_id = Uuid::parse_str(&entry.org_id).ok()?;
                let event_id = &Uuid::parse_str(&entry.event_id).ok()?;
                let org = actions::orgs::find_org_by_uid(&mut conn, &org_id).ok()??;

                let event = actions::events::find_event_by_uid(&mut conn, &event_id).ok()??;

                let secret = get_org_secret(&mut conn, &org_id).ok()?;

                Some(Invite {
                    event: event,
                    org: org,
                    secret: secret.unwrap_or("".into()),
                })
            })
            .collect();

        Ok(invite_queue_entries)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(success))
}
