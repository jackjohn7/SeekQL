use std::path::PathBuf;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    info!("Initializing router...");

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/register", get(register))
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

async fn login() -> impl IntoResponse {
    let template = LoginTemplate{};
    debug!("Rendering page login");
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

async fn register() -> impl IntoResponse {
    let template = RegisterTemplate{};
    debug!("Rendering page register");
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "signup.html")]
struct RegisterTemplate;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
