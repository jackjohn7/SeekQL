use std::{path::PathBuf, sync::Arc};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use askama::Template;
use axum::routing::post;
use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod html_template;
mod problems;

use crate::html_template::HtmlTemplate;

pub struct AppState{
    pub db: Arc<Pool<Postgres>>
}

use crate::controllers::auth_controller::{login, register, register_email};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set up tracing for logging with defaults
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seek_ql=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Connecting to main database");
    let pool = PgPoolOptions::new()
        //.max_connections(5)
        .connect("postgres://seekql:password@localhost:5432").await?;

    info!("Running migrations");
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await?;

    info!("Initializing application state");

    let app_state = Arc::new(AppState{ db: Arc::new(pool) });

    info!("Initializing router...");

    let app = Router::new()
        .route("/", get(index))
        // Auth routes
        .route("/login", get(login))
        .route("/register", get(register))
        .route("/register/email", post(register_email))
        .with_state(app_state)
        .nest_service(
            "/public/styles",
            ServeDir::new(format!(
                "{}/public/styles",
                std::env::current_dir().unwrap()
                    .to_str()
                    .unwrap())))
        .nest_service(
            "/favicon.ico",
            ServeFile::new(std::env::current_dir().unwrap().as_path().join(PathBuf::from("public/favicon.ico")).to_str().unwrap())
        );


    let port = 5173_u16;
    let addr = std::net::SocketAddr::from(([0 ,0 ,0 , 0], port));

    info!("Router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to listen");
    axum::serve(listener, app).await.expect("Failed to serve router");

    Ok(())
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate{name: "bruv"};
    debug!("Rendering page root");
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str
}

