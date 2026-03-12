use super::DbError;
use crate::models::{self, TemplateEntry};
use diesel::prelude::*;
use uuid::Uuid;

pub fn set_template(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
    s_template_type: &str,
    s_template_variant: &str,
    s_content: Option<&str>,
) -> Result<(), DbError> {
    use crate::schema::templates::dsl::*;

    diesel::delete(
        templates
            .filter(event.eq(s_event.to_string()))
            .filter(template_type.eq(s_template_type))
            .filter(template_variant.eq(s_template_variant)),
    )
    .execute(conn)?;

    if let Some(s_content) = s_content {
        diesel::insert_into(templates)
            .values(TemplateEntry {
                event: s_event.to_string(),
                template_type: s_template_type.to_string(),
                template_variant: s_template_variant.to_string(),
                content: s_content.to_string(),
            })
            .execute(conn)?;
    }

    Ok(())
}

pub fn get_template(
    conn: &mut SqliteConnection,
    s_event: &Uuid,
    s_template_type: &str,
    s_template_variant: &str,
) -> Result<Option<String>, DbError> {
    use crate::schema::templates::dsl::*;

    Ok(templates
        .select(TemplateEntry::as_select())
        .filter(event.eq(s_event.to_string()))
        .filter(template_type.eq(s_template_type))
        .filter(template_variant.eq(s_template_variant))
        .first::<models::TemplateEntry>(conn)
        .optional()?
        .map(|r| r.content))
}
