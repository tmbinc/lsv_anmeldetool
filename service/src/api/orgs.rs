use crate::models::{self, Org};
use crate::{actions, DbPool, ErrorResponse};
use actix_cors::Cors;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::{Json, Path};
use actix_web::ResponseError;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer, Responder};
use apistos::actix::CreatedJson;
use apistos::api_operation;
use apistos::app::OpenApiWrapper;
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::web::{get, post, put, resource, scope};
use apistos::ApiComponent;
use apistos::ApiErrorComponent;
use core::fmt::Formatter;
use diesel::{prelude::*, r2d2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::net::Ipv4Addr;
use uuid::Uuid;

/// Finds org by UID.
///
#[api_operation(summary = "get one org by ID")]
pub async fn get_org(
    pool: web::Data<DbPool>,
    user_uid: Path<Uuid>,
) -> Result<Json<Org>, ErrorResponse> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_org_by_uid(&mut conn, user_uid)
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    match user {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => Ok(Json(user)),

        // user was not found; return 404 response with error message
        None => Err(ErrorResponse::NotFound(format!(
            "No user found with UID: {user_uid}"
        ))),
    }
}

#[api_operation(summary = "update org")]
pub async fn update_org(
    pool: web::Data<DbPool>,
    user_uid: Path<Uuid>,
    data: Json<Org>,
) -> Result<Json<Org>, ErrorResponse> {
    println!("update!");
    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::update_org(&mut conn, *user_uid, data.into_inner())
    })
    .await
    .unwrap() // fixme
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)
    .unwrap(); // fixme

    // match user {
    //     // user was found; return 200 response with JSON formatted user object
    //     Some(user) => Ok(Json(user)),

    //     // user was not found; return 404 response with error message
    //     None => Err(ErrorResponse::NotFound(format!(
    //         "No user found with UID: {user_uid}"
    //     ))),
    // }
    Ok(Json(user))
}

/// List orgs
#[api_operation(summary = "get list of orgs")]
pub async fn get_orgs(pool: web::Data<DbPool>) -> Result<Json<Vec<Org>>, ErrorResponse> {
    let orgs = web::block(move || {
        let mut conn = pool.get()?;

        actions::list_org(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(Json(orgs))
}

#[api_operation(summary = "add an org")]
pub async fn add_org(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewOrg>,
) -> Result<Json<Org>, ErrorResponse> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let org = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_org(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(Json(org))
}
