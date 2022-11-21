use askama::Template;
use axum::{
    extract::{Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use time::format_description;
use time::format_description::FormatItem;
use crate::data::visitor::{Visitor, latest_visitors};
use crate::data::get_conn;

#[derive(Deserialize)]
pub struct Pagination {
    pn: Option<i64>,
    ps: Option<i64>,
}

pub async fn adm_visitors(Query(pagination): Query<Pagination>) -> impl IntoResponse {
    let pool = get_conn().await.unwrap();
    let conn = pool.get().await.unwrap();
    let pn = pagination.pn.unwrap_or(0);
    let ps = pagination.ps.unwrap_or(10);
    let vs = latest_visitors(&conn, pn, ps)
        .await.unwrap();
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]",
    ).unwrap();

    let template = HelloTemplate {
        pn,
        ps,
        visitors: vs,
        time_format: format,
    };
    crate::apis::HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "adm_visitors.html")]
struct HelloTemplate<'a> {
    visitors: Vec<Visitor>,
    pn: i64,
    ps: i64,
    time_format: Vec<FormatItem<'a>>,
}

