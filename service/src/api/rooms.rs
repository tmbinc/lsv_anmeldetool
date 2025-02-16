use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{Role, Room};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SetRoomsPayload {
    rooms: Vec<Room>,
}

#[api_operation(summary = "set rooms for a given event + group", skip_args = "user")]
pub async fn set_rooms(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<(Uuid, Uuid)>,
    data: Json<SetRoomsPayload>,
) -> Result<Json<()>, ErrorResponse> {
    let mut data = data.into_inner().rooms;
    let (event_uid, group_uid) = path.into_inner();

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
        }

        actions::rooms::set_rooms(&mut conn, &event_uid, &group_uid, &data)?;
        Ok(())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}

#[api_operation(summary = "get rooms for a given event", skip_args = "user")]
pub async fn get_rooms(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<Uuid>,
) -> Result<Json<Vec<Room>>, ErrorResponse> {
    let event_uid = path.into_inner();

    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let data = web::block(move || -> Result<Vec<Room>, DbError> {
        let mut conn = pool.get()?;

        Ok(actions::rooms::get_rooms(&mut conn, &event_uid)?)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}
