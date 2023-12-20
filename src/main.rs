use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

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
        .route("/", get(hello));
    let port = 5173_u16;
    let addr = std::net::SocketAddr::from(([0 ,0 ,0 , 0], port));

    info!("Router initialized, now listening on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to listen");

    axum::serve(listener, app).await.expect("Failed to serve router");

    Ok(())
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate{name: "bruv"};
    debug!("Rendering page root");
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str
}

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
