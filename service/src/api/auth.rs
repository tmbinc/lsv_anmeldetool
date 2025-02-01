use actix_identity::Identity;
use actix_web::dev::Payload;
use actix_web::web::{Json, Path};
use actix_web::{error, web, Error, FromRequest, HttpMessage as _, HttpRequest, HttpResponse};
use apistos::{api_operation, ApiComponent};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Deserialize;
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::actions::org_secrets::check_org_auth_token;
use crate::errors::ErrorResponse;
use crate::models::{Role, SlimUser, User};
use crate::utils::verify;
use crate::DbPool;

#[derive(Debug, Deserialize, ApiComponent, JsonSchema)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Ok(Self {
            email: "".to_string(),
            role: Role::None,
        }))
    }
}

#[api_operation(summary = "logout", skip_args = "id")]
pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}

#[api_operation(summary = "login")]
pub async fn login(
    req: HttpRequest,
    auth_data: web::Json<AuthData>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || query(auth_data.into_inner(), pool)).await??;

    let user_string = serde_json::to_string(&user).unwrap();
    Identity::login(&req.extensions(), user_string).unwrap();

    Ok(HttpResponse::NoContent().finish())
}

#[derive(Deserialize, ApiComponent, JsonSchema)]
pub struct OrgAuthLoginData {
    token: String,
}

#[api_operation(summary = "login for org")]
pub async fn org_login(
    req: HttpRequest,
    org_id: Path<Uuid>,
    auth_data: web::Json<OrgAuthLoginData>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    let org_id = org_id.into_inner();
    let token = auth_data.into_inner().token;
    let valid_token = web::block(move || {
        let mut conn = pool.get()?;
        check_org_auth_token(&mut conn, org_id, &token)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if valid_token {
        let user = LoggedUser {
            email: "Org".to_string(),
            role: Role::Org(org_id),
        };
        let user_string = serde_json::to_string(&user).unwrap();
        Identity::login(&req.extensions(), user_string).unwrap();
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(ErrorResponse::Unauthorized("invalid token".into()))
    }
}

#[api_operation(summary = "get logged in identity", skip_args = "logged_user")]
pub async fn get_me(logged_user: LoggedUser) -> Result<Json<LoggedUser>, ErrorResponse> {
    Ok(Json(logged_user))
}
/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<DbPool>) -> Result<SlimUser, ErrorResponse> {
    use crate::schema::users::dsl::{email, users};

    let mut conn = pool.get().unwrap();

    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(&mut conn)
        .map_err(error::ErrorInternalServerError)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ErrorResponse::Unauthorized("query".into()))
}
