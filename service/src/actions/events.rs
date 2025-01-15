use crate::actions::DbError;
use crate::models;
use diesel::prelude::*;
use uuid::Uuid;

pub fn find_event_by_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
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

pub fn list_events(conn: &mut SqliteConnection) -> Result<Vec<models::Event>, DbError> {
    use crate::schema::events::dsl::*;

    Ok(events.select(models::Event::as_select()).load(conn)?)
}

pub fn insert_new_event(
    conn: &mut SqliteConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<models::Event, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::events::dsl::*;

    let new_event = models::Event {
        id: Uuid::new_v4().to_string(),
        name: nm.to_string(),
        public: false,
        begin: None,
    };

    diesel::insert_into(events)
        .values(&new_event)
        .execute(conn)?;

    Ok(new_event)
}
