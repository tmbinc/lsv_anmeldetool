use super::DbError;
use crate::models::QuestionnaireAnswer;
use crate::schema::questionnaire_answers;
use crate::{models::Questionnaire, schema::questionnaire};
use diesel::prelude::*;
use diesel::SqliteConnection;
use uuid::Uuid;

pub fn update_questionnaire(
    conn: &mut SqliteConnection,
    data: &Questionnaire,
) -> Result<(), DbError> {
    diesel::insert_into(questionnaire::table)
        .values(data)
        .on_conflict(questionnaire::id)
        .do_update()
        .set(data)
        .execute(conn)?;

    Ok(())
}

pub fn delete_questionnaire(conn: &mut SqliteConnection, id: &Uuid) -> Result<(), DbError> {
    diesel::delete(questionnaire::table.filter(questionnaire::id.eq(id.to_string())))
        .execute(conn)?;

    Ok(())
}

pub fn create_questionnaire(
    conn: &mut SqliteConnection,
    event_uid: &Uuid,
) -> Result<Questionnaire, DbError> {
    let questionnaire = Questionnaire {
        id: Uuid::new_v4().to_string(),
        event_id: Some(event_uid.to_string()),
        org_id: None,
        question_text: "".into(),
        question_type: "".into(),
        question_data: "".into(),
    };

    diesel::insert_into(questionnaire::table)
        .values(&questionnaire)
        .execute(conn)?;

    Ok(questionnaire)
}

pub fn get_questionnaire(
    conn: &mut SqliteConnection,
    event_id: Option<&Uuid>,
    org_id: Option<&Uuid>,
) -> Result<Vec<Questionnaire>, DbError> {
    let mut res = Vec::<Questionnaire>::new();

    let mut query = questionnaire::table.into_boxed();

    if let Some(event_id) = event_id {
        query = query.filter(questionnaire::event_id.eq(event_id.to_string()));
    }

    if let Some(org_id) = org_id {
        query = query.filter(questionnaire::org_id.eq(org_id.to_string()));
    }

    res.append(&mut query.select(Questionnaire::as_select()).load(conn)?);

    Ok(res)
}

pub fn get_questionnaire_answers(
    conn: &mut SqliteConnection,
    event_id: &Uuid,
    org_id: &Uuid,
) -> Result<Vec<QuestionnaireAnswer>, DbError> {
    let query = questionnaire_answers::table
        .filter(questionnaire_answers::event_id.eq(event_id.to_string()))
        .filter(questionnaire_answers::org_id.eq(org_id.to_string()));

    let res = query.select(QuestionnaireAnswer::as_select()).load(conn)?;

    Ok(res)
}

pub fn update_questionnaire_answer(
    conn: &mut SqliteConnection,
    data: &QuestionnaireAnswer,
) -> Result<(), DbError> {
    diesel::insert_into(questionnaire_answers::table)
        .values(data)
        .on_conflict((
            questionnaire_answers::question_id,
            questionnaire_answers::org_id,
            questionnaire_answers::event_id,
        ))
        .do_update()
        .set(data)
        .execute(conn)?;

    Ok(())
}
