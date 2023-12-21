use std::sync::Arc;

use askama::Template;
use axum::{
    routing::get,
    response::IntoResponse,
    Router, extract::State,
};

use tracing::debug;

use crate::{html_template::HtmlTemplate, AppState};


pub fn register_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/login", get(login))
        .route("/register", get(register))
}

async fn login(State(AppState{ dummy, .. }): State<AppState>) -> impl IntoResponse {
    let template = LoginTemplate{};
    debug!("Rendering page login, {}", dummy);
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

async fn register(State(state): State<AppState>) -> impl IntoResponse {
    debug!("dummy data from state: {}", state.dummy);
    let template = RegisterTemplate{};
    debug!("Rendering page register");
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "signup.html")]
struct RegisterTemplate;
