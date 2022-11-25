use askama::Template;
use axum::{
    extract::{Form},
    response::{IntoResponse},
};
use axum::extract::{Query, State};
use axum::headers::HeaderValue;
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::AppendHeaders;
use serde::Deserialize;
use crate::data::visitor;
use crate::apis::{AppState, DatabaseConnection};
use crate::apis::auth::{gen_access_token, gen_exchange_token};

#[derive(Deserialize)]
pub struct TicketParams {
    ticket: String,
}

pub async fn save_session_get(
    State(s): State<AppState>,
    Query(tp): Query<TicketParams>) -> impl IntoResponse {
    println!("ticket: {}", tp.ticket);
    let token = gen_access_token(s.session_secret.as_bytes(), tp.ticket);
    (
        StatusCode::FOUND,
        AppendHeaders([
            (LOCATION, "/adm/visitors".to_string()),
            (SET_COOKIE, format!("x-token={};path=/", token)),
        ]),
        "success"
    )
}

pub async fn login_get() -> impl IntoResponse {
    let template = PageTemplate {};
    crate::apis::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

pub async fn login_post(
    State(s): State<AppState>,
    Form(v): Form<LoginParams>) -> impl IntoResponse {
    let token = gen_exchange_token(s.session_secret.as_bytes(), &v.username);
    (StatusCode::OK,
     AppendHeaders([
         ("HX-Redirect", format!("{}{}", "/adm/exchange?ticket=", token)),
     ])
    )
}

#[derive(Template)]
#[template(path = "login.html")]
struct PageTemplate {}

