use crate::actions::groups::get_group_by_id;
use crate::actions::DbError;
use crate::errors::ErrorResponse;
use crate::models::{Group, NewGroup};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::api_operation;
use uuid::Uuid;

#[api_operation(summary = "get list of groups")]
pub async fn get_groups(
    pool: web::Data<DbPool>,
    event_id: Path<Uuid>,
) -> Result<Json<Vec<Group>>, ErrorResponse> {
    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::list_groups(&mut conn, &event_id.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add a group")]
pub async fn add_group(
    pool: web::Data<DbPool>,
    event_id: Path<Uuid>,
    new_group: Json<NewGroup>,
) -> Result<Json<Group>, ErrorResponse> {
    let group = web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::create_group(&mut conn, &event_id.into_inner(), new_group.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(group))
}

#[api_operation(summary = "update group")]
pub async fn update_group(
    pool: web::Data<DbPool>,
    data: Json<Group>,
) -> Result<Json<String>, ErrorResponse> {
    web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::update_group(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json("updated".to_owned()))
}

#[api_operation(summary = "delete a group")]
pub async fn delete_group(
    pool: web::Data<DbPool>,
    group_uid: Path<Uuid>,
) -> Result<Json<String>, ErrorResponse> {
    let group_uid = group_uid.into_inner();
    let group = web::block(move || -> Result<Option<usize>, DbError> {
        let mut conn = pool.get()?;

        let group = get_group_by_id(&mut conn, &group_uid)?;

        if let Some(group) = group {
            Ok(Some(actions::groups::delete_group(&mut conn, group)?))
        } else {
            Ok(None)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match group {
        Some(1) => Ok(Json("ok".to_owned())),
        _ => Err(ErrorResponse::NotFound(format!(
            "No group found with UID: {group_uid}"
        ))),
    }
}
