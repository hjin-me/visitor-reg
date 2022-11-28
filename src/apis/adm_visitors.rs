use std::env::temp_dir;
use std::fs;
use askama::Template;
use axum::{
    extract::{Query},
    response::{IntoResponse},
};
use axum::http::header::CONTENT_DISPOSITION;
use axum::http::StatusCode;
use axum::response::AppendHeaders;
use serde::Deserialize;
use time::format_description;
use time::format_description::FormatItem;
use tracing::trace;
use xlsxwriter::Workbook;
use crate::apis::{DatabaseConnection};
use crate::apis::auth::AuthSession;
use crate::data::visitor::{Visitor, latest_visitors};

#[derive(Deserialize)]
pub struct Pagination {
    pn: Option<i64>,
    ps: Option<i64>,
}


pub async fn adm_visitors(
    DatabaseConnection(pool): DatabaseConnection,
    AuthSession(s): AuthSession,
    Query(pagination): Query<Pagination>) -> impl IntoResponse {
    let pn = pagination.pn.unwrap_or(0);
    let ps = pagination.ps.unwrap_or(20);
    let conn = pool.get().await.unwrap();
    let vs = latest_visitors(&conn, pn, ps)
        .await.unwrap();
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]",
    ).unwrap();

    trace!("adm_visitors: pn={}, ps={}, {:?}", pn, ps, s);
    let template = HelloTemplate {
        pn,
        ps,
        visitors: vs,
        time_format: format,
    };
    (
        StatusCode::OK,
        crate::apis::HtmlTemplate(template),
    )
}

#[derive(Template)]
#[template(path = "adm_visitors.html")]
struct HelloTemplate<'a> {
    visitors: Vec<Visitor>,
    pn: i64,
    ps: i64,
    time_format: Vec<FormatItem<'a>>,
}

pub async fn export_visitors(
    DatabaseConnection(pool): DatabaseConnection,
    AuthSession(_): AuthSession,
) -> impl IntoResponse {
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]",
    ).unwrap();

    let conn = pool.get().await.unwrap();
    let vs = latest_visitors(&conn, 0, 1000)
        .await.unwrap();
    let filename = temp_dir().join(
        format!("attachment; filename=\"visitors_{}.xlsx\"",
                time::OffsetDateTime::now_utc().unix_timestamp())).to_str()
        .unwrap().to_string();

    let workbook = Workbook::new(&filename).unwrap();
    // let format1 = workbook.add_format().set_font_color(xlsxwriter::FormatColor::Red);

    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    // 客姓名	电话	公司	拜访时间	拜访对象
    sheet1.write_string(0, 0, "访客姓名", None).unwrap();
    sheet1.write_string(0, 1, "电话", None).unwrap();
    sheet1.write_string(0, 2, "公司", None).unwrap();
    sheet1.write_string(0, 3, "拜访时间", None).unwrap();
    sheet1.write_string(0, 4, "拜访对象", None).unwrap();
    let mut row = 0;
    for visitor in vs {
        row += 1;
        sheet1.write_string(row, 0, &visitor.appellation, None).unwrap();
        sheet1.write_string(row, 1, &visitor.mobile_phone_no, None).unwrap();
        sheet1.write_string(row, 2, &visitor.company, None).unwrap();
        sheet1.write_string(row, 3, &visitor.last_visit_date.format(&format).unwrap(), None).unwrap();
        sheet1.write_string(row, 4, &visitor.invited_by, None).unwrap();
    }
    // sheet1.write_datetime(1, 1, 0., None).unwrap();
    // sheet1.write_url(
    //     1,
    //     1,
    //     "https://github.com/informationsea/xlsxwriter-rs",
    //     Some(&format2),
    // )?;
    // sheet1.merge_range(2, 0, 3, 2, "Hello, world", Some(&format3))?;

    // sheet1.set_selection(1, 0, 1, 2);
    // sheet1.set_tab_color(FormatColor::Cyan);
    workbook.close().unwrap();


    let f = fs::read(&filename).unwrap();

    (
        StatusCode::OK,
        AppendHeaders([
            (CONTENT_DISPOSITION, format!("attachment; filename=\"visitors_{}.xlsx\"", time::OffsetDateTime::now_utc().unix_timestamp()))
        ]),
        f
    )
}
