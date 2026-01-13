use crate::actions::DbError;
use crate::models;
use diesel::prelude::*;
use uuid::Uuid;

pub fn find_event_by_uid(
    conn: &mut SqliteConnection,
    uid: &Uuid,
) -> Result<Option<models::Event>, DbError> {
    use crate::schema::events::dsl::*;

    Ok(events
        .filter(id.eq(uid.to_string()))
        .first::<models::Event>(conn)
        .optional()?)
}

pub fn update_event(
    conn: &mut SqliteConnection,
    uid: Uuid,
    data: models::Event,
) -> Result<models::Event, DbError> {
    use crate::schema::events::dsl::*;

    diesel::update(events.filter(id.eq(uid.to_string())))
        .set(&data)
        .execute(conn)?;
    Ok(data)
}

pub fn list_events(
    conn: &mut SqliteConnection,
    only_public: bool,
) -> Result<Vec<models::Event>, DbError> {
    use crate::schema::events::dsl::*;

    if only_public {
        Ok(events
            .select(models::Event::as_select())
            .filter(public.eq(true))
            .load(conn)?)
    } else {
        Ok(events.select(models::Event::as_select()).load(conn)?)
    }
}

pub fn insert_new_event(conn: &mut SqliteConnection, nm: &str) -> Result<models::Event, DbError> {
    use crate::schema::events::dsl::*;

    let new_event = models::Event {
        id: Uuid::new_v4().to_string(),
        name: nm.to_string(),
        public: false,
        begin: None,
        public_reg_until: None,
        description: "".to_string(),
        allow_set_present: false,
        allow_user_changes: true,
        results_are_public: false,
        self_registration_allowed: false,
        registration_active: false,
        registration_start_date: None,
    };

    diesel::insert_into(events)
        .values(&new_event)
        .execute(conn)?;

    Ok(new_event)
}
