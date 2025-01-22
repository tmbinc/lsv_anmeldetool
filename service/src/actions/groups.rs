use crate::actions::DbError;
use crate::models::{self, Group, NewGroup};
use crate::schema;
use diesel::prelude::*;
use uuid::Uuid;

// pub fn list_groups_by_event(
//     conn: &mut SqliteConnection,
//     event_uid: Uuid,
// ) -> Result<Option<Vec<Group>>, DbError> {
//     let the_event = find_event_by_uid(conn, event_uid)?;

//     match the_event {
//         Some(the_event) => {
//             use crate::schema::groups::dsl::*;
//             let group_event = GroupEvent::belonging_to(&the_event)
//                 .inner_join(groups)
//                 .select(Group::as_select())
//                 .load(conn)?;
//             Ok(Some(group_event))
//         }
//         None => Ok(None),
//     }
// }

pub fn get_group_by_id(
    conn: &mut SqliteConnection,
    group_uid: &Uuid,
) -> Result<Option<Group>, DbError> {
    use crate::schema::groups::dsl::*;

    Ok(groups
        .filter(id.eq(group_uid.to_string()))
        .first::<models::Group>(conn)
        .optional()?)
}

pub fn list_groups(conn: &mut SqliteConnection, event_uid: &Uuid) -> Result<Vec<Group>, DbError> {
    use crate::schema::groups::dsl::*;

    Ok(groups
        .filter(event_id.eq(event_uid.to_string()))
        .select(Group::as_select())
        .load(conn)?)
}

pub fn update_group(conn: &mut SqliteConnection, group: Group) -> Result<(), DbError> {
    diesel::insert_into(schema::groups::table)
        .values(&group)
        .on_conflict(schema::groups::id)
        .do_update()
        .set(&group)
        .execute(conn)?;
    Ok(())
}

pub fn delete_group(conn: &mut SqliteConnection, group: Group) -> Result<usize, DbError> {
    let res = diesel::delete(&group).execute(conn)?;

    Ok(res)
}

pub fn create_group(
    conn: &mut SqliteConnection,
    event_id: &Uuid,
    group: NewGroup,
) -> Result<Group, DbError> {
    let group = Group {
        id: Uuid::new_v4().to_string(),
        event_id: event_id.to_string(),
        name: group.name,
    };
    // TODO: verify that event actually exists
    diesel::insert_into(schema::groups::table)
        .values(&group)
        .execute(conn)?;
    Ok(group)
}
