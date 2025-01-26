use crate::actions::events::find_event_by_uid;
use crate::actions::DbError;
use crate::models::{Event, Org, OrgEvent};
use crate::schema::org_event;
use diesel::prelude::*;
use uuid::Uuid;

use super::orgs::find_org_by_uid;

pub fn list_event_orgs(
    conn: &mut SqliteConnection,
    event_uid: Uuid,
) -> Result<Option<Vec<Org>>, DbError> {
    let event = find_event_by_uid(conn, &event_uid)?;

    match event {
        Some(event) => {
            use crate::schema::orgs::dsl::*;
            let event_org_list = OrgEvent::belonging_to(&event)
                .inner_join(orgs)
                .select(Org::as_select())
                .load(conn)?;

            Ok(Some(event_org_list))
        }
        None => Ok(None),
    }
}

pub fn list_org_events(
    conn: &mut SqliteConnection,
    org_uid: Uuid,
) -> Result<Option<Vec<Event>>, DbError> {
    let org = find_org_by_uid(conn, org_uid)?;

    match org {
        Some(org) => {
            use crate::schema::events::dsl::*;
            let org_event_list = OrgEvent::belonging_to(&org)
                .inner_join(events)
                .select(Event::as_select())
                .load(conn)?;

            Ok(Some(org_event_list))
        }
        None => Ok(None),
    }
}

pub fn update_event_org_state(
    conn: &mut SqliteConnection,
    org_event: OrgEvent,
) -> Result<(), DbError> {
    diesel::insert_into(org_event::table)
        .values(&org_event)
        .on_conflict((org_event::event_id, org_event::org_id))
        .do_update()
        .set(&org_event)
        .execute(conn)?;
    Ok(())
}

pub fn delete_event_org(conn: &mut SqliteConnection, org_event: OrgEvent) -> Result<(), DbError> {
    diesel::delete(&org_event).execute(conn)?;
    Ok(())
}
