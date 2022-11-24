use askama::Template;
use axum::{
    extract::{Form},
    response::{IntoResponse},
};
use serde::Deserialize;
use crate::data::visitor;
use crate::apis::{DatabaseConnection};

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
    DatabaseConnection(pool): DatabaseConnection,
    Form(v): Form<NewVisitorParams>) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();
    visitor::new_visitor(&conn, &v.appellation, &v.mobile_phone_no, &v.company, &v.invited_by).await.unwrap();
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

