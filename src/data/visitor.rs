use bb8::{PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use time::{OffsetDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Visitor {
    pub id: Uuid,
    pub name: String,
    pub mobile_phone_no: String,
    pub last_visit_date: OffsetDateTime,
}

async fn get_one_visitor(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
) -> Result<Visitor, String> {
    let row = conn
        .query_one("select id, name, mobile_phone_no, last_visit_at from visitors limit 1", &[])
        .await
        .map_err(db_error)?;
    // let x: Timestamp<Instant> = row.try_get(2).map_err(internal_error)?;
    let r: Visitor = Visitor {
        id: row.try_get(0).map_err(db_error)?,
        name: row.try_get(1).map_err(db_error)?,
        mobile_phone_no: row.try_get(2).map_err(db_error)?,
        last_visit_date: row.try_get(3).map_err(db_error)?,
    };

    Ok(r)
}

async fn new_visitor(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    v: Visitor,
) -> Result<String, String> {
    let u = Uuid::new_v4();
    let r = conn
        .execute(
            "insert into visitors (id, name, mobile_phone_no, last_visit_at) values ($1, $2, $3, $4)",
            &[&u, &v.name, &v.mobile_phone_no, &v.last_visit_date],
        )
        .await
        .map_err(db_error)?;
    if r == 1 {
        Ok("success".to_string())
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
            "select id, name, mobile_phone_no, last_visit_at from visitors
                                                order by last_visit_at desc limit $1 offset $2",
            &[&size, &offset])
        .await
        .unwrap();
    let mut r: Vec<Visitor> = Vec::new();

    for x in rows {
        r.push(Visitor {
            id: x.try_get(0).unwrap(),
            name: x.try_get(1).unwrap(),
            mobile_phone_no: x.try_get(2).unwrap(),
            last_visit_date: x.try_get(3).unwrap(),
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
        let pool = crate::data::get_conn().await.unwrap();
        let conn = pool.get().await.unwrap();
        println!("{:?}", new_visitor(&conn, Visitor {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            mobile_phone_no: "1234567890".to_string(),
            last_visit_date: OffsetDateTime::now_utc(),
        }).await.unwrap());
    }

    #[tokio::test]
    async fn test_query() {
        let pool = crate::data::get_conn().await.unwrap();
        let conn = pool.get().await.unwrap();
        println!("{:?} ", latest_visitors(&conn, 0, 3).await.unwrap());
        // assert_eq!(1, 2)
        // assert_eq!("aaaa", get_one_visitor(&conn).await.unwrap().name);
    }

    #[tokio::test]
    async fn test_query_one() {
        let pool = crate::data::get_conn().await.unwrap();
        let conn = pool.get().await.unwrap();
        println!("{:?} ", get_one_visitor(&conn).await.unwrap());
        assert_eq!("aaaa", get_one_visitor(&conn).await.unwrap().name);
    }
}
