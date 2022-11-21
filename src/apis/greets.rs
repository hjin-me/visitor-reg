use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    crate::apis::HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

