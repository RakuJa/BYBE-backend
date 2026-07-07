mod routes;
pub mod sanitizer;

use crate::routes::{bestiary, encounter, hazard, health, npc, shareable, shop};
use actix_cors::Cors;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};
use bybe::AppState;
use bybe::db;
use bybe::models::shared::game_system_enum::GameSystem;
use dotenvy::{dotenv, from_path};
use postgresql_embedded::{PostgreSQL, SettingsBuilder};
use sqlx::postgres::PgPoolOptions;
use sqlx::{AssertSqlSafe, Connection};
use std::env;
use std::num::NonZero;
use tokio::sync::oneshot;
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt, reload};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(index))]
struct ApiDoc;

#[derive(Default)]
pub enum StartupState {
    #[default]
    Clean,
    Persistent,
}

#[derive(Default)]
pub enum InitializeLogResponsibility {
    Delegated,
    #[default]
    Personal,
}

impl From<String> for StartupState {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "CLEAN" => Self::Clean,
            "PERSISTENT" => Self::Persistent,
            _ => Self::default(),
        }
    }
}

#[derive(Default)]
pub struct StartOptions {
    pub env_location: Option<String>,
    pub sql_location: Option<String>,
    pub jsons_location: Option<(String, String)>,
    pub db_data_dir: Option<String>,
    pub shutdown_signal: Option<oneshot::Receiver<()>>,
    pub init_log_resp: InitializeLogResponsibility,
    pub startup_state_override: Option<StartupState>,
    pub ready_signal: Option<std::sync::mpsc::Sender<()>>,
}

const DEFAULT_DB_DATA_DIR: &str = "./.postgres-data";

/// Whether the embedded PostgreSQL data directory has already been populated.
///
/// Callers that manage their own startup lifecycle (e.g. the Tauri app) use this
/// to decide whether this is a first-time setup.
pub fn db_initialized(db_data_dir: &str) -> bool {
    std::path::Path::new(db_data_dir).exists()
}

#[utoipa::path(get, path = "/")]
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world! You should not be here.. What are you looking for?")
}

fn get_nickname_json_path() -> String {
    env::var("NICKNAMES_PATH").expect("Error fetching nickname json")
}

fn get_sql_dump() -> String {
    env::var("SQL_PATH").expect("Error fetching sql dump")
}

fn get_name_json_path() -> String {
    env::var("NAMES_PATH").expect("Error fetching name json")
}

fn get_service_ip() -> String {
    env::var("SERVICE_IP").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn get_service_startup_state() -> StartupState {
    env::var("SERVICE_STARTUP_STATE").unwrap_or_default().into()
}

fn get_service_port() -> u16 {
    env::var("SERVICE_PORT")
        .ok()
        .map_or(25566, |port| port.parse().unwrap_or(25566))
}

fn get_db_password() -> String {
    env::var("DB_PASSWORD").unwrap_or_else(|_| "bybe-local-only".to_string())
}

fn get_service_workers() -> usize {
    let available_cpus =
        usize::from(std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap()));
    env::var("N_OF_SERVICE_WORKERS")
        .ok()
        .map_or(available_cpus, |n_of_workers| {
            n_of_workers.parse().unwrap_or(available_cpus)
        })
}

fn init_docs(openapi: utoipa::openapi::OpenApi) -> utoipa::openapi::OpenApi {
    openapi
        .merge_from(health::init_docs())
        .merge_from(shareable::init_docs())
        .nest("/pf", bestiary::pf_init_docs())
        .nest("/pf", encounter::pf_init_docs())
        .nest("/pf", shop::pf_init_docs())
        .nest("/pf", npc::pf_init_docs())
        .nest("/pf", hazard::pf_init_docs())
        .nest("/sf", bestiary::sf_init_docs())
        .nest("/sf", encounter::sf_init_docs())
        .nest("/sf", shop::sf_init_docs())
        .nest("/sf", npc::sf_init_docs())
        .nest("/sf", hazard::sf_init_docs())
}

#[actix_web::main]
pub async fn start(options: StartOptions) -> std::io::Result<()> {
    if let Some(env_path) = &options.env_location {
        from_path(env_path).ok();
    } else {
        dotenv().ok();
    }
    let _guard; // to let it live for all the application, otherwise it won't write to file
    let stdout_reload_handle;
    match options.init_log_resp {
        InitializeLogResponsibility::Personal => {
            let file_appender = rolling::daily("./logs", "bybe.log");
            let (file_writer, guard) = non_blocking(file_appender);
            _guard = guard;

            let (stdout_filter, handle) = reload::Layer::new(EnvFilter::new("info"));
            stdout_reload_handle = Some(handle);
            tracing_subscriber::registry()
                .with(
                    fmt::layer()
                        .with_writer(file_writer)
                        .with_filter(EnvFilter::new("warn")),
                )
                .with(
                    fmt::layer()
                        .with_writer(std::io::stdout)
                        .with_filter(stdout_filter),
                )
                .init();
        }
        InitializeLogResponsibility::Delegated => {
            stdout_reload_handle = None;
        }
    }
    let (name_json_path, nick_json_path) = options
        .jsons_location
        .unwrap_or_else(|| (get_name_json_path(), get_nickname_json_path()));
    let service_ip = get_service_ip();
    let service_port = get_service_port();
    let startup_state: StartupState = options
        .startup_state_override
        .unwrap_or_else(get_service_startup_state);
    let service_workers = get_service_workers();
    let db_data_dir = options
        .db_data_dir
        .unwrap_or_else(|| DEFAULT_DB_DATA_DIR.to_string());

    info!("Starting DB connection");

    if matches!(startup_state, StartupState::Clean) {
        let db_data_path = std::path::Path::new(&db_data_dir);
        if db_data_path.exists() {
            std::fs::remove_dir_all(db_data_path).expect("Failed to clean db data directory");
        }
    }
    let mut postgresql = PostgreSQL::new(
        SettingsBuilder::new()
            .data_dir(db_data_dir)
            .host("127.0.0.1")
            // 0 = pick a free port at startup, avoiding conflicts with any
            // system-wide PostgreSQL the user might already have running.
            .port(0)
            // set static pwd
            .username("postgres")
            .password(get_db_password())
            .temporary(false)
            .timeout(Some(std::time::Duration::from_secs(120)))
            .build(),
    );
    postgresql
        .setup()
        .await
        .expect("Failed to set up embedded PostgreSQL");
    postgresql
        .start()
        .await
        .expect("Failed to start embedded PostgreSQL");
    let db_uri = postgresql.settings().url("postgres");
    if matches!(startup_state, StartupState::Clean) {
        if let Some(ref handle) = stdout_reload_handle {
            let _ = handle.reload(EnvFilter::new("info,sqlx=off"));
        }
        let sql_path = options.sql_location.unwrap_or_else(get_sql_dump);
        let dump_sql = std::fs::read_to_string(sql_path)?;
        let dump_sql: String = dump_sql
            .lines()
            .filter(|line| !line.starts_with('\\'))
            .collect::<Vec<_>>()
            .join("\n");
        {
            let mut conn = sqlx::PgConnection::connect(&db_uri)
                .await
                .expect("failed to connect to db server");
            sqlx::raw_sql(AssertSqlSafe(dump_sql))
                .execute(&mut conn)
                .await
                .expect("Failed to load data from SQL dump into database");
            sqlx::query("SET search_path TO public")
                .execute(&mut conn)
                .await
                .expect("Failed to reset search_path after dump load");
        } // conn dropped here, releasing the single init connection

        let init_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_uri)
            .await
            .expect("Failed to create init pool");

        info!("Starting DB PF2E Table cleanup & creation of update CORE tables");
        db::cr_core_initializer::update_creature_core_table(&init_pool, GameSystem::Pathfinder)
            .await
            .expect("Could not initialize correctly core creature table.. Startup failed");

        info!("Starting DB SF2E Table cleanup & creation of update CORE tables");
        db::cr_core_initializer::update_creature_core_table(&init_pool, GameSystem::Starfinder)
            .await
            .expect("Could not initialize correctly core creature table.. Startup failed");
        if let Some(ref handle) = stdout_reload_handle {
            let _ = handle.reload(EnvFilter::new("info"));
        }
    }

    if let Some(ready_signal) = options.ready_signal {
        let _ = ready_signal.send(());
    }

    info!(
        "starting HTTP server at http://{}:{}",
        service_ip.as_str(),
        service_port
    );

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_uri)
        .await
        .expect("Failed to create runtime pool");

    // Swagger initialization
    let openapi = init_docs(ApiDoc::openapi());
    // Configure endpoints
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(CacheControl(vec![
                        CacheDirective::Private,
                        CacheDirective::MaxAge(u32::MAX),
                    ]))
                    .add(("X-Content-Type-Options", "nosniff")),
            )
            .service(index)
            .service(
                web::scope("/pf")
                    .configure(bestiary::pf_init_endpoints)
                    .configure(encounter::pf_init_endpoints)
                    .configure(shop::pf_init_endpoints)
                    .configure(npc::pf_init_endpoints)
                    .configure(hazard::pf_init_endpoints),
            )
            .service(
                web::scope("/sf")
                    .configure(bestiary::sf_init_endpoints)
                    .configure(encounter::sf_init_endpoints)
                    .configure(shop::sf_init_endpoints)
                    .configure(npc::sf_init_endpoints)
                    .configure(hazard::sf_init_endpoints),
            )
            .configure(health::init_endpoints)
            .configure(shareable::init_endpoints)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                name_json_path: name_json_path.clone(),
                nick_json_path: nick_json_path.clone(),
            }))
    })
    .workers(service_workers)
    .bind((service_ip, service_port))?
    .run();

    if let Some(shutdown_signal) = options.shutdown_signal {
        let server_handle = server.handle();
        actix_web::rt::spawn(async move {
            let _ = shutdown_signal.await;
            server_handle.stop(true).await;
        });
    }

    let x = server.await;
    postgresql
        .stop()
        .await
        .expect("Failed to stop embedded PostgreSQL during shutdown");
    x
}
