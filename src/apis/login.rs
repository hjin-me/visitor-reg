use askama::Template;
use axum::{
    extract::{Form},
    response::{IntoResponse},
};
use axum::extract::{Query, State};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::AppendHeaders;
use serde::Deserialize;
use url::form_urlencoded::byte_serialize;
use crate::apis::{AppState};
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

pub async fn login_get(
    Query(q): Query<MsgParams>,
) -> impl IntoResponse {
    let template = PageTemplate { msg: q.msg.unwrap_or("".to_string()) };
    crate::apis::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct MsgParams {
    msg: Option<String>,
}

pub async fn login_post(
    State(s): State<AppState>,
    Form(v): Form<LoginParams>,
) -> impl IntoResponse {
    if v.username != s.allowed_uid || v.password != s.allowed_password {
        let msg: String = byte_serialize("登陆失败".as_bytes()).collect();
        return (StatusCode::OK,
                AppendHeaders([
                    ("HX-Redirect", format!("{}{}", "/adm/in?msg=", msg)),
                ])
        );
    }
    let token = gen_exchange_token(s.session_secret.as_bytes(), &v.username);
    let q: String = byte_serialize(token.as_bytes()).collect();
    (StatusCode::OK,
     AppendHeaders([
         ("HX-Redirect", format!("{}{}", "/adm/exchange?ticket=", q)),
     ])
    )
}

#[derive(Template)]
#[template(path = "login.html")]
struct PageTemplate {
    msg: String,
}

