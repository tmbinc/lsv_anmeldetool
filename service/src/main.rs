use actix_cors::Cors;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{error, middleware, web, App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::web::{get, post, put, resource, scope};
use apistos::ApiErrorComponent;
use core::fmt::Formatter;
use diesel::{prelude::*, r2d2};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

mod actions;
mod api;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

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
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        todo!()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "An API".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .document(spec)
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            // add route handlers
            .service(
                scope("/api/v1")
                    .service(
                        resource("org/{org}")
                            .route(get().to(api::orgs::get_org))
                            .route(put().to(api::orgs::update_org)),
                    )
                    .service(resource("orgs").route(get().to(api::orgs::get_orgs)))
                    .service(resource("org").route(post().to(api::orgs::add_org))),
            )
            .build("/openapi.json")
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
