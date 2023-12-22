use std::sync::Arc;

use askama::Template;
use axum::{
    response::IntoResponse,
    extract::State, Form
};

use serde::Deserialize;

use tracing::debug;

use crate::{html_template::HtmlTemplate, AppState};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

pub async fn login(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let dummy = state.dummy;
    let template = LoginTemplate{};
    debug!("Rendering page login, {}", dummy);
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "signup.html")]
struct RegisterTemplate;

pub async fn register(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    debug!("dummy data from state: {}", state.dummy);
    let template = RegisterTemplate{};
    debug!("Rendering page register");
    HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct Email {
    email: String,
}

#[derive(Template)]
#[template(path = "signup_email.html")]
struct RegisterEmailTemplate{
    error_msg: Option<String>,
}

pub async fn register_email(State(_): State<Arc<AppState>>, Form(form): Form<Email>) -> impl IntoResponse {
    debug!("Rendering page register");
    if form.email == "test@test.com" {
        let template = RegisterEmailTemplate{ error_msg: None };
        return HtmlTemplate(template);
    } else {
        let template = RegisterEmailTemplate{ error_msg: Some("Invalid email provided, bro".to_owned()) };
        return HtmlTemplate(template);
    }
}
