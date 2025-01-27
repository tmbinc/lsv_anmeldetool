use uuid::Uuid;

pub mod auth;
pub mod events;
pub mod groups;
pub mod orgs;
pub mod teams;

enum Authentication {
    None,
    Admin,
    Org(Uuid),
}
