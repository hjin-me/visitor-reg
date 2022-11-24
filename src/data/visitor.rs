use bb8::{PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use time::{OffsetDateTime, macros::{offset}};
use uuid::Uuid;

#[derive(Debug)]
pub struct Visitor {
    pub id: Uuid,
    pub appellation: String,
    pub company: String,
    pub mobile_phone_no: String,
    pub last_visit_date: OffsetDateTime,
    pub invited_by: String,
}

async fn get_one_visitor(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
) -> Result<Visitor, String> {
    let row = conn
        .query_one("select id, appellation, mobile_phone_no, last_visit_at, company, invited_by from visitors limit 1", &[])
        .await
        .map_err(db_error)?;
    // let x: Timestamp<Instant> = row.try_get(2).map_err(internal_error)?;
    let r: Visitor = Visitor {
        id: row.try_get(0).map_err(db_error)?,
        appellation: row.try_get(1).map_err(db_error)?,
        company: row.try_get(4).map_err(db_error)?,
        mobile_phone_no: row.try_get(2).map_err(db_error)?,
        last_visit_date: row.try_get(3).map_err(db_error)?,
        invited_by: row.try_get(5).map_err(db_error)?,
    };

    Ok(r)
}

pub async fn new_visitor(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    appellation: &String,
    mobile_phone_no: &String,
    company: &String,
    invited_by: &String,
) -> Result<(), String> {
    let u = Uuid::new_v4();
    let now = OffsetDateTime::now_utc().to_offset(offset!(+8));
    let r = conn
        .execute(
            "insert into visitors (id, appellation, mobile_phone_no, last_visit_at, company, invited_by) values ($1, $2, $3, $4, $5, $6)",
            &[&u, &appellation, &mobile_phone_no, &now, &company, &invited_by],
        )
        .await
        .map_err(db_error)?;
    if r == 1 {
        Ok(())
    } else {
        Err("Failed to insert".to_string())
    }
}

pub async fn latest_visitors(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    offset: i64, size: i64,
) -> Result<Vec<Visitor>, String> {
    let rows = conn
        .query(
            "select id, appellation, mobile_phone_no, last_visit_at, company, invited_by from visitors
                                                order by last_visit_at desc limit $1 offset $2",
            &[&size, &offset])
        .await
        .unwrap();
    let mut r: Vec<Visitor> = Vec::new();

    for x in rows {
        let v = Visitor {
            id: x.try_get(0).unwrap(),
            appellation: x.try_get(1).unwrap(),
            mobile_phone_no: x.try_get(2).unwrap(),
            last_visit_date: x.try_get(3).unwrap(),
            company: x.try_get(4).unwrap(),
            invited_by: x.try_get(5).unwrap(),
        };
        r.push(Visitor {
            id: v.id,
            appellation: v.appellation,
            company: v.company,
            mobile_phone_no: v.mobile_phone_no,
            last_visit_date: v.last_visit_date.to_offset(offset!(+8)),
            invited_by: v.invited_by,
        });
    }
    Ok(r)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn db_error<E>(err: E) -> String
    where
        E: std::error::Error,
{
    err.to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[tokio::test]
    async fn test_add() {
        let pg_dsn: String = String::from("host=localhost user=postgres password=example");
        let pool = crate::data::get_pool(&pg_dsn).await.unwrap();
        let conn = pool.get().await.unwrap();
        println!("{:?}", new_visitor(&conn,
                                     &"test".to_string(),
                                     &"1234567890".to_string(),
                                     &"mouge".to_string(),
                                     &"ss".to_string(),
        ).await.unwrap());
    }

    #[tokio::test]
    async fn test_query() {
        let pg_dsn: String = String::from("host=localhost user=postgres password=example");
        let pool = crate::data::get_pool(&pg_dsn).await.unwrap();
        let conn = pool.get().await.unwrap();
        println!("{:?} ", latest_visitors(&conn, 0, 3).await.unwrap());
        // assert_eq!(1, 2)
        // assert_eq!("aaaa", get_one_visitor(&conn).await.unwrap().name);
    }

    #[tokio::test]
    async fn test_query_one() {
        let pg_dsn: String = String::from("host=localhost user=postgres password=example");
        let pool = crate::data::get_pool(&pg_dsn).await.unwrap();
        let conn = pool.get().await.unwrap();
        println!("{:?} ", get_one_visitor(&conn).await.unwrap());
        assert_eq!("aaaa", get_one_visitor(&conn).await.unwrap().appellation);
    }
}
