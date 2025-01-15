use crate::models::{Event, NewEvent};
use crate::{actions, DbPool, ErrorResponse};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::api_operation;
use uuid::Uuid;

/// Finds event by UID.
#[api_operation(summary = "get one event by ID")]
pub async fn get_event(
    pool: web::Data<DbPool>,
    user_uid: Path<Uuid>,
) -> Result<Json<Event>, ErrorResponse> {
    let user_uid = user_uid.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::find_event_by_uid(&mut conn, user_uid)
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

#[api_operation(summary = "update event")]
pub async fn update_event(
    pool: web::Data<DbPool>,
    user_uid: Path<Uuid>,
    data: Json<Event>,
) -> Result<Json<Event>, ErrorResponse> {
    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::update_event(&mut conn, *user_uid, data.into_inner())
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    Ok(Json(user))
}

/// List events
#[api_operation(summary = "get list of events")]
pub async fn get_events(pool: web::Data<DbPool>) -> Result<Json<Vec<Event>>, ErrorResponse> {
    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::events::list_events(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add an event")]
pub async fn add_event(
    pool: web::Data<DbPool>,
    form: web::Json<NewEvent>,
) -> Result<Json<Event>, ErrorResponse> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let org = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::events::insert_new_event(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(Json(org))
}
