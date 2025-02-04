use crate::actions::DbError;
use crate::api::auth::LoggedUser;
use crate::errors::ErrorResponse;
use crate::models::{EventOrgState, Questionnaire, QuestionnaireAnswer, Role};
use crate::{actions, DbPool};
use actix_web::web::{Json, Path};
use actix_web::{error, web};
use apistos::api_operation;
use uuid::Uuid;

#[api_operation(summary = "get questionaire for event", skip_args = "user")]
pub async fn get_questionnaire_for_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<Vec<Questionnaire>>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let success = web::block(move || -> Result<Vec<Questionnaire>, DbError> {
        let mut conn = pool.get()?;
        let questionnaire =
            actions::questionnaire::get_questionnaire(&mut conn, Some(&event_uid), None)?;

        Ok(questionnaire)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(success))
}

#[api_operation(summary = "create questionaire for event", skip_args = "user")]
pub async fn create_questionnaire_for_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    event_uid: Path<Uuid>,
) -> Result<Json<Questionnaire>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let event_uid = event_uid.into_inner();

    let success = web::block(move || -> Result<Questionnaire, DbError> {
        let mut conn = pool.get()?;
        let questionnaire = actions::questionnaire::create_questionnaire(&mut conn, &event_uid)?;

        Ok(questionnaire)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(success))
}

#[api_operation(summary = "delete questionaire", skip_args = "user")]
pub async fn delete_questionnaire(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    questionnaire_uid: Path<Uuid>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let questionnaire_uid = questionnaire_uid.into_inner();

    web::block(move || -> Result<(), DbError> {
        let mut conn = pool.get()?;

        actions::questionnaire::delete_questionnaire(&mut conn, &questionnaire_uid)?;

        Ok(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json("deleted".into()))
}

#[api_operation(summary = "update questionaire", skip_args = "user")]
pub async fn update_questionnaire(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    data: Json<Questionnaire>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    web::block(move || -> Result<(), DbError> {
        let mut conn = pool.get()?;
        actions::questionnaire::update_questionnaire(&mut conn, &data)?;

        Ok(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json("ok".into()))
}

#[api_operation(summary = "get questionaire for org + event", skip_args = "user")]
pub async fn get_questionnaire_for_org_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_event_uid: Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Questionnaire>>, ErrorResponse> {
    let (org_uid, event_uid) = org_event_uid.into_inner();
    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}

        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let success = web::block(move || -> Result<Option<Vec<Questionnaire>>, DbError> {
        let mut conn = pool.get()?;

        let event_org_state =
            actions::org_event::get_event_org_state(&mut conn, &event_uid, &org_uid)?
                .unwrap_or(EventOrgState::NotEnlisted);

        if !match event_org_state {
            EventOrgState::NotEnlisted => false,
            EventOrgState::Registered => false,
            EventOrgState::Invited => true,
            EventOrgState::Updated => true,
            EventOrgState::Submitted => true,
            EventOrgState::Verified => true,
        } {
            Ok(None)
        } else {
            // Return all questions for specifically this event + org
            let mut questionnaire = actions::questionnaire::get_questionnaire(
                &mut conn,
                Some(&event_uid),
                Some(&org_uid),
            )?;

            //... and all that are just for this event (and all orgs).
            questionnaire.append(&mut actions::questionnaire::get_questionnaire(
                &mut conn,
                Some(&event_uid),
                None,
            )?);

            Ok(Some(questionnaire))
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match success {
        Some(success) => Ok(Json(success)),
        None => Err(ErrorResponse::Unauthorized(
            "org not enlisted in event".to_string(),
        )),
    }
}

#[api_operation(
    summary = "get questionaire answers for org + event",
    skip_args = "user"
)]
pub async fn get_questionnaire_answer_for_org_event(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    org_event_uid: Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<QuestionnaireAnswer>>, ErrorResponse> {
    let (org_uid, event_uid) = org_event_uid.into_inner();
    match user.role {
        Role::Admin => {}
        Role::Org(org) if org == org_uid => {}

        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let success = web::block(
        move || -> Result<Option<Vec<QuestionnaireAnswer>>, DbError> {
            let mut conn = pool.get()?;

            let event_org_state =
                actions::org_event::get_event_org_state(&mut conn, &event_uid, &org_uid)?
                    .unwrap_or(EventOrgState::NotEnlisted);

            if !match event_org_state {
                EventOrgState::NotEnlisted => false,
                EventOrgState::Registered => false,
                EventOrgState::Invited => true,
                EventOrgState::Updated => true,
                EventOrgState::Submitted => true,
                EventOrgState::Verified => true,
            } {
                Ok(None)
            } else {
                Ok(Some(actions::questionnaire::get_questionnaire_answers(
                    &mut conn, &event_uid, &org_uid,
                )?))
            }
        },
    )
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match success {
        Some(success) => Ok(Json(success)),
        None => Err(ErrorResponse::Unauthorized(
            "org not enlisted in event".to_string(),
        )),
    }
}

#[api_operation(summary = "update questionaire answer", skip_args = "user")]
pub async fn update_questionnaire_answer(
    pool: web::Data<DbPool>,
    user: LoggedUser,
    data: Json<QuestionnaireAnswer>,
) -> Result<Json<String>, ErrorResponse> {
    let data = data.into_inner();
    match user.role {
        Role::Admin => {}
        Role::Org(org) if org.to_string() == data.org_id => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    let allowed = web::block(move || -> Result<bool, DbError> {
        let mut conn = pool.get()?;

        let event_uid = Uuid::parse_str(&data.event_id)?;
        let org_uid = Uuid::parse_str(&data.org_id)?;

        let event_org_state =
            actions::org_event::get_event_org_state(&mut conn, &event_uid, &org_uid)?
                .unwrap_or(EventOrgState::NotEnlisted);

        if !match event_org_state {
            EventOrgState::NotEnlisted => false,
            EventOrgState::Registered => false,
            EventOrgState::Invited => true,
            EventOrgState::Updated => true,
            EventOrgState::Submitted => true,
            EventOrgState::Verified => true,
        } {
            Ok(false)
        } else {
            actions::questionnaire::update_questionnaire_answer(&mut conn, &data)?;
            Ok(true)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if allowed {
        Ok(Json("ok".into()))
    } else {
        Err(ErrorResponse::Unauthorized(
            "org not enlisted in event".to_string(),
        ))
    }
}
