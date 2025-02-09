use crate::actions::groups::get_group_by_id;
use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{Group, NewGroup, Role};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::api_operation;
use uuid::Uuid;

#[api_operation(summary = "get list of groups", skip_args = "user")]
pub async fn get_groups(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_id: Path<Uuid>,
) -> Result<Json<Vec<Group>>, ErrorResponse> {
    match user.role {
        Role::Admin | Role::None | Role::Org(_) => {}
    };

    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::list_groups(&mut conn, &event_id.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "get group by id", skip_args = "user")]
pub async fn get_group(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    group_id: Path<Uuid>,
) -> Result<Json<Group>, ErrorResponse> {
    match user.role {
        Role::Admin | Role::None | Role::Org(_) => {}
    };

    let group_id = group_id.into_inner();

    let group = web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::get_group_by_id(&mut conn, &group_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match group {
        Some(group) => Ok(Json(group)),
        None => Err(ErrorResponse::NotFound(format!(
            "No group found with UID: {group_id}"
        ))),
    }
}

#[api_operation(summary = "add a group", skip_args = "user")]
pub async fn add_group(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_id: Path<Uuid>,
    new_group: Json<NewGroup>,
) -> Result<Json<Group>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let group = web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::create_group(&mut conn, &event_id.into_inner(), new_group.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(group))
}

#[api_operation(summary = "update group", skip_args = "user")]
pub async fn update_group(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    data: Json<Group>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    web::block(move || {
        let mut conn = pool.get()?;

        actions::groups::update_group(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json("updated".to_owned()))
}

#[api_operation(summary = "delete a group", skip_args = "user")]
pub async fn delete_group(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    group_uid: Path<Uuid>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

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
