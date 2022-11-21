use askama::Template;
use axum::{
    extract::{Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use crate::data::visitor::{Visitor, latest_visitors};
use crate::data::get_conn;

#[derive(Deserialize)]
pub struct Pagination {
    pn: i64,
    ps: i64,
}

pub async fn adm_visitors(Query(pagination): Query<Pagination>) -> impl IntoResponse {
    let pool = get_conn().await.unwrap();
    let conn = pool.get().await.unwrap();
    let vs = latest_visitors(&conn, pagination.pn, pagination.ps)
        .await.unwrap();

    let template = HelloTemplate {
        pn: pagination.pn,
        ps: pagination.ps,
        visitors: vs,
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "adm_visitors.html")]
struct HelloTemplate {
    visitors: Vec<Visitor>,
    pn: i64,
    ps: i64,
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
