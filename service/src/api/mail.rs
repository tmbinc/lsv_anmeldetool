use actix_web::web::Json;
use actix_web::{error, web};
use apistos::{api_operation, ApiComponent};
use lettre::message::header::ContentType;
use lettre::message::{Body, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::read_to_string;
use std::sync::Arc;
use uuid::Uuid;

use crate::actions::invite_queue::{delete_invite, get_one_invite};
use crate::actions::org_secrets::get_org_secret;
use crate::actions::{self, DbError};
use crate::errors::ErrorResponse;
use crate::models::Role;
use crate::DbPool;

use super::auth::LoggedUser;
use super::orgs::Invite;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub user: String,
    pub pwd: String,
    pub server: String,
    pub from: String,
}

impl Config {
    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = read_to_string(filename)
            .map_err(|err| format!("Unable to read config file: {}", err))?;
        let config: Config = toml::from_str(&config_str)
            .map_err(|err| format!("Unable to parse config file: {}", err))?;
        Ok(config)
    }
}

#[derive(JsonSchema, Deserialize, ApiComponent)]
pub struct MailConfirm {
    send: bool,
}

#[api_operation(
    summary = "send a single invite",
    skip_args = "user",
    skip_args = "config"
)]
pub async fn send_one_invite(
    pool: web::Data<DbPool>,
    config: web::Data<Arc<Config>>,
    user: LoggedUser,
    confirm: web::Json<MailConfirm>,
) -> Result<Json<String>, ErrorResponse> {
    match user.role {
        Role::Admin => {}
        _ => {
            return Err(ErrorResponse::Unauthorized("".to_string()));
        }
    };

    // This is really just so we have a non-empty POST content.
    if !confirm.send {
        return Err(ErrorResponse::Unauthorized("need confirmation".to_string()));
    }

    let result = web::block(move || -> Result<Json<String>, ErrorResponse> {
        let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;

        let mut invite = || -> Result<Option<Invite>, DbError> {
            if let Some(entry) = get_one_invite(&mut conn)? {
                let org_id = Uuid::parse_str(&entry.org_id)?;
                let event_id = &Uuid::parse_str(&entry.event_id)?;
                let org = actions::orgs::find_org_by_uid(&mut conn, &org_id)?;

                let event = actions::events::find_event_by_uid(&mut conn, &event_id)?;

                let secret = get_org_secret(&mut conn, &org_id)?;

                if let Some(org) = org {
                    if let Some(event) = event {
                        if let Some(secret) = secret {
                            return Ok(Some(Invite { event, org, secret }));
                        }
                    }
                }
            }

            Ok(None)
        };

        // Obtain the invite.
        let invite = invite().map_err(error::ErrorInternalServerError)?;

        // If there was one, send the mail.
        if let Some(invite) = invite {
            let to_email = invite
                .org
                .contact_email
                .ok_or(ErrorResponse::Internal(format!(
                    "invite org {} does not have contact_email",
                    invite.org.id
                )))?;
            let to_name = invite.org.contact_name;
            let subject = format!("Anmeldung: {}", invite.event.name);
            let body = format!(
                "Hallo, {}!

Vielen Dank für die Registrierung bei dem Turnier \"{}\".

Zur Anmeldung der Mannschaften benutzen Sie bitte den folgenden Link:

https://anmeldung.lsv1873.de/org/{}/{}/{}

Vielen Dank!",
                invite.org.name, invite.event.name, invite.org.id, invite.event.id, invite.secret
            );

            let mailer = SmtpTransport::relay(&config.server)
                .expect("Failed to parse relay server provided in Config.toml")
                .credentials(Credentials::from((config.user.clone(), config.pwd.clone())))
                .build();

            let email = Message::builder()
                .from(
                    config
                        .from
                        .parse::<Mailbox>()
                        .expect("failed to parse from"),
                )
                .to(Mailbox::new(
                    to_name,
                    to_email.parse().map_err(|_| {
                        ErrorResponse::Internal(format!("invalid email: {:?}", to_email))
                    })?,
                ))
                .header(ContentType::TEXT_PLAIN)
                .subject(subject)
                .body(Body::new(body))
                .expect("failed to build message");

            println!("sending {:?}", email);

            let email_res = match mailer.send(&email) {
                Ok(_) => {
                    info!("Message to {} sent", to_email);

                    delete_invite(
                        &mut conn,
                        &Uuid::parse_str(&invite.org.id).unwrap(),
                        &Uuid::parse_str(&invite.event.id).unwrap(),
                    )
                    .map_err(error::ErrorInternalServerError)?;

                    Ok(Json("Message sent successfully".into()))
                }
                Err(e) => {
                    error!("FAILED to send message to {}", to_email);
                    Err(ErrorResponse::Internal(format!(
                        "Failed to send message: {}",
                        e.to_string()
                    )))
                }
            };

            email_res
        } else {
            Ok(Json("No invites in queue".into()))
        }
    })
    .await?;

    result
}
