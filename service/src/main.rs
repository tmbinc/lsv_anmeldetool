use std::sync::Arc;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::web::{delete, get, post, put, resource, scope};
use diesel::{prelude::*, r2d2};

mod actions;
mod api;
mod errors;
mod models;
mod schema;
mod utils;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(
    connection: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    {
        let mut connection = pool.get().expect("DB connection failed for migration");
        run_migrations(&mut connection).expect("failed to run migrations");
    }

    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a 16 bit int");
    let path = std::env::var("STATIC_FILE_PATH").expect("STATIC_FILE_PATH must be set");
    let static_files = String::from(path.strip_suffix("/").unwrap_or(&path));

    let secret_key = Key::generate();

    let email_config_file = std::env::var("EMAIL_CONFIG")
        .ok()
        .unwrap_or("Email.toml".into());

    let email_config = Arc::new(
        api::mail::Config::load_from_file(&email_config_file)
            .expect("Failed to load email configuration file"),
    );

    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "LSV REG API".to_string(),
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
            // Add email config
            .app_data(web::Data::new(email_config.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            // Authentication
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("session".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                    .build(),
            )
            // add route handlers
            .service(
                scope("/api/v1")
                    .service(
                        resource("org/{org}")
                            .route(get().to(api::orgs::get_org))
                            .route(put().to(api::orgs::update_org)),
                    )
                    .service(resource("org/{org}/auth").route(post().to(api::auth::org_login)))
                    .service(
                        resource("orgs/self_register")
                            .route(post().to(api::orgs::self_register_org)),
                    )
                    .service(
                        resource("org/{org}/events").route(get().to(api::orgs::get_org_events)),
                    )
                    .service(
                        resource("org/{org}/{event}/teams")
                            .route(get().to(api::teams::get_org_teams)),
                    )
                    .service(resource("org/{org}/{event}/questionnaire_answer").route(
                        get().to(api::questionnaire::get_questionnaire_answer_for_org_event),
                    ))
                    .service(
                        resource("org/{org}/{event}/invite")
                            .route(post().to(api::orgs::invite_org_event)),
                    )
                    .service(
                        resource("invites")
                            .route(get().to(api::orgs::list_invites))
                            .route(post().to(api::mail::send_one_invite)),
                    )
                    .service(
                        resource("org/{org}/{event}")
                            .route(get().to(api::orgs::get_org_event_state)),
                    )
                    .service(
                        resource("org/{org}/{event}/questionnaire")
                            .route(get().to(api::questionnaire::get_questionnaire_for_org_event)),
                    )
                    .service(resource("orgs").route(get().to(api::orgs::get_orgs)))
                    .service(resource("org").route(post().to(api::orgs::add_org)))
                    .service(
                        resource("event/{event}")
                            .route(get().to(api::events::get_event))
                            .route(put().to(api::events::update_event)),
                    )
                    .service(
                        resource("event/{event}/questionnaire_answers").route(
                            get().to(api::questionnaire::get_questionnaire_answers_for_event),
                        ),
                    )
                    .service(
                        resource("event/{event}/pairings/{group}/{round}")
                            .route(put().to(api::pairings::set_pairings)),
                    )
                    .service(
                        resource("event/{event}/results/{group}/{round}")
                            .route(put().to(api::results::set_results)),
                    )
                    .service(
                        resource("event/{event}/timetable")
                            .route(put().to(api::timetable::set_timetable))
                            .route(get().to(api::timetable::get_timetable)),
                    )
                    .service(
                        resource("event/{event}/rooms/{group}")
                            .route(put().to(api::rooms::set_rooms)),
                    )
                    .service(resource("event/{event}/rooms").route(get().to(api::rooms::get_rooms)))
                    .service(
                        resource("event/{event}/pairings")
                            .route(get().to(api::pairings::get_pairings)),
                    )
                    .service(
                        resource("event/{event}/results")
                            .route(get().to(api::results::get_results)),
                    )
                    .service(
                        resource("event/{event}/questionnaire")
                            .route(get().to(api::questionnaire::get_questionnaire_for_event))
                            .route(post().to(api::questionnaire::create_questionnaire_for_event)),
                    )
                    .service(
                        resource("event/{event}/groups")
                            .route(get().to(api::groups::get_groups))
                            .route(post().to(api::groups::add_group)),
                    )
                    .service(resource("group").route(put().to(api::groups::update_group)))
                    .service(
                        resource("group/{group}")
                            .route(delete().to(api::groups::delete_group))
                            .route(get().to(api::groups::get_group)),
                    )
                    .service(
                        resource("event/{event}/orgs").route(get().to(api::events::get_event_orgs)),
                    )
                    .service(
                        resource("team")
                            .route(post().to(api::teams::add_team))
                            .route(put().to(api::teams::update_team)),
                    )
                    .service(
                        resource("team/{team}")
                            .route(get().to(api::teams::get_team))
                            .route(delete().to(api::teams::delete_team)),
                    )
                    .service(
                        resource("team/{team}/present")
                            .route(put().to(api::teams::set_team_present)),
                    )
                    .service(
                        resource("event/{event}/org/{org}")
                            .route(put().to(api::events::set_event_org_state)),
                    )
                    .service(resource("events").route(get().to(api::events::get_events)))
                    .service(resource("event").route(post().to(api::events::add_event)))
                    .service(
                        resource("/auth")
                            .route(post().to(api::auth::login))
                            .route(delete().to(api::auth::logout))
                            .route(get().to(api::auth::get_me)),
                    )
                    .service(
                        resource("questionnaire/{id}")
                            .route(delete().to(api::questionnaire::delete_questionnaire)),
                    )
                    .service(
                        resource("questionnaire")
                            .route(put().to(api::questionnaire::update_questionnaire)),
                    )
                    .service(
                        resource("questionnaire_answer")
                            .route(put().to(api::questionnaire::update_questionnaire_answer)),
                    ),
            )
            .build("/openapi.json")
            .service(
                actix_files::Files::new("/", static_files.clone())
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(
                            vec![static_files.clone(), "index.html".to_string()].join("/"),
                        )
                        .expect("index file should exist"),
                    ),
            )
    })
    .bind(("0.0.0.0", port))?
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
        .max_size(1)
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
