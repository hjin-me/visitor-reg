use askama::Template;
use axum::{
    extract::{Query, Form},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum::extract::State;
use serde::Deserialize;
use uuid::Uuid;
use crate::data::visitor;
use crate::data::get_pool;
use crate::data::visitor::Visitor;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct NewVisitorParams {
    appellation: String,
    company: String,
    invited_by: String,
    mobile_phone_no: String,
}


pub async fn new_visitor_get() -> impl IntoResponse {
    let template = PageTemplate {};
    crate::apis::HtmlTemplate(template)
}

pub async fn new_visitor_post(
    State(pool): State<Pool<PostgresConnectionManager<NoTls>>>,
    Form(v): Form<NewVisitorParams>) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();
    visitor::new_visitor(&conn, &v.appellation,
                         &v.company,
                         &v.invited_by,
                         &v.mobile_phone_no,
    ).await.unwrap();
    let template = AfterPostTemplate {
        appellation: v.appellation,
    };
    crate::apis::HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "new_visitor.html")]
struct PageTemplate {}

#[derive(Template)]
#[template(path = "new_visitor_post.html")]
struct AfterPostTemplate {
    appellation: String,
}

