use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{Group, Role, TimetableEntry};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SetTimetablePayload {
    timetable: Vec<TimetableEntry>,
}

#[api_operation(
    summary = "set timetable for a given event + group",
    skip_args = "user"
)]
pub async fn set_timetable(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    path: Path<Uuid>,
    data: Json<SetTimetablePayload>,
) -> Result<Json<()>, ErrorResponse> {
    let mut data = data.into_inner().timetable;
    let event_uid = path.into_inner();

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
        }

        actions::timetable::set_timetable(&mut conn, &event_uid, &data)?;
        Ok(())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(data))
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct TimetableRow {
    pub group: String,
    pub row_index: i32,
    pub name: String,
    pub expected_time: NaiveDateTime,
    pub last_update: NaiveDateTime,
    pub state: String,
}

#[derive(Serialize, ApiComponent, JsonSchema)]
pub struct TimetableForEvent {
    event_name: String,
    rows: Vec<TimetableRow>,
    groups: Vec<Group>,
}

#[api_operation(summary = "get timetable for a given event", skip_args = "user")]
pub async fn get_timetable(
    pool: web::Data<DbPool>,
    _user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<TimetableForEvent>, ErrorResponse> {
    let event_uid = event_uid.into_inner();
    let data = web::block(move || -> Result<Option<TimetableForEvent>, DbError> {
        let mut conn = pool.get()?;

        let Some(event) = actions::events::find_event_by_uid(&mut conn, &event_uid)? else {
            return Ok(None);
        };

        let timetable = actions::timetable::get_timetable(&mut conn, &event_uid)?;
        let groups = actions::groups::list_groups(&mut conn, &event_uid)?;

        let timetable_rows = timetable
            .into_iter()
            .map(|entry| TimetableRow {
                group: entry.group_id,
                row_index: entry.row_index,
                name: entry.name,
                expected_time: entry.expected_time,
                last_update: entry.last_update,
                state: entry.state,
            })
            .collect();

        Ok(Some(TimetableForEvent {
            event_name: event.name,
            rows: timetable_rows,
            groups,
        }))
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    match data {
        Some(data) => Ok(Json(data)),
        None => Err(ErrorResponse::NotFound(format!("Event not found"))),
    }
}
