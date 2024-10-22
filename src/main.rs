use actix_web::dev::Response;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use sqlx::{database, PgPool, Pool, Postgres};
use tracing::level_filters::LevelFilter;
use tracing::{debug, error, info, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use crate::endpoints::authentication::create_user;

mod endpoints;
mod models;
type DynError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, DynError>;


async fn db_init() -> BoxResult<Pool<Postgres>> {
    let db_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL found!");
    debug!(db_url);
    info!("Initialising database connection!");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(256)
        .min_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
#[actix_web::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let debug_file =
        tracing_appender::rolling::hourly("./logs/", "debug").with_max_level(Level::TRACE);

    let warn_file =
        tracing_appender::rolling::hourly("./logs/", "warnings").with_max_level(Level::WARN);
    let all_files = debug_file.and(warn_file);

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env()
                .expect("Unable to read log level"),
        )
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(all_files)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_ansi(true)
                .with_writer(std::io::stdout.with_max_level(Level::TRACE))
                .with_file(true)
                .with_line_number(true),
        )
        .init();
    let database = db_init().await;

    match database {
        Ok(db) => {
            let _ = HttpServer::new(move || {
                App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .app_data(web::Data::new(db.clone())) // primitive clone
                    .service(create_user)
            })
            .bind(("127.0.0.1", 4444))
            .expect("Unable to start webserver")
            .run()
            .await;
        }
        Err(e) => {
            error!("Unable to connect to database with error {}", e);
        }
    }
}
