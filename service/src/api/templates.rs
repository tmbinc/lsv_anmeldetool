use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::Role;
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SetTimetablePayload {
    content: Option<String>,
}

#[api_operation(summary = "set template for given type for event", skip_args = "user")]
pub async fn set_template(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<(Uuid, String, String)>,
    data: Json<SetTimetablePayload>,
) -> Result<Json<()>, ErrorResponse> {
    let data = data.into_inner().content;
    let (event_uid, template_type, template_variant) = path.into_inner();

    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let data = web::block(move || -> Result<(), DbError> {
        let mut conn = pool.get()?;

        actions::templates::set_template(
            &mut conn,
            &event_uid,
            &template_type,
            &template_variant,
            data.as_deref(),
        )?;
        Ok(())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}

#[api_operation(
    summary = "get template for a given event and type",
    skip_args = "user"
)]
pub async fn get_template(
    pool: web::Data<DbPool>,
    _user: LoggedUser,
    path: Path<(Uuid, String, String)>,
) -> Result<Json<String>, ErrorResponse> {
    let (event_uid, template_type, template_variant) = path.into_inner();

    let data = web::block(move || -> Result<Option<String>, DbError> {
        let mut conn = pool.get()?;

        Ok(actions::templates::get_template(
            &mut conn,
            &event_uid,
            &template_type,
            &template_variant,
        )?)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    match data {
        Some(data) => Ok(Json(data)),
        None => Err(ErrorResponse::NotFound(format!("template not found"))),
    }
}
