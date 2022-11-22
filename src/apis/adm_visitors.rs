use std::sync::Arc;
use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse},
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::Deserialize;
use time::format_description;
use time::format_description::FormatItem;
use tokio_postgres::NoTls;
use crate::data::visitor::{Visitor, latest_visitors};
use crate::data::get_pool;

#[derive(Deserialize)]
pub struct Pagination {
    pn: Option<i64>,
    ps: Option<i64>,
}


pub async fn adm_visitors(
    State(pool): State<Pool<PostgresConnectionManager<NoTls>>>,
    Query(pagination): Query<Pagination>) -> impl IntoResponse {
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

