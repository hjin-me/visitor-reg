use askama::Template;
use axum::{
    extract::{Query, Form},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use uuid::Uuid;
use crate::data::visitor;
use crate::data::get_conn;
use crate::data::visitor::Visitor;

#[derive(Deserialize)]
pub struct NewVisitor {
    appellation: String,
    company: String,
    invited_by: String,
    mobile_phone_no: String,

}

pub async fn new_visitor_page() -> impl IntoResponse {
    let template = PageTemplate {};
    crate::apis::HtmlTemplate(template)
}

pub async fn new_visitor(Form(v): Form<NewVisitor>) -> impl IntoResponse {
    let pool = get_conn().await.unwrap();
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

