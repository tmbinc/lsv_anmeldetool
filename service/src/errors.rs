use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{error, middleware, web, App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::web::{delete, get, post, put, resource, scope};
use apistos::ApiErrorComponent;
use core::fmt::Formatter;
use diesel::{prelude::*, r2d2};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
#[openapi_error(
    status(code = 403),
    status(code = 404),
    status(code = 405, description = "Invalid input"),
    status(code = 409)
)]
pub enum ErrorResponse {
    MethodNotAllowed(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    Other,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        match self {
            ErrorResponse::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            ErrorResponse::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorResponse::Conflict(_) => StatusCode::CONFLICT,
            ErrorResponse::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ErrorResponse::Other => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<actix_web::Error> for ErrorResponse {
    fn from(_value: actix_web::Error) -> Self {
        Self::Other
    }
}
impl From<error::BlockingError> for ErrorResponse {
    fn from(_value: error::BlockingError) -> Self {
        Self::Other
    }
}
