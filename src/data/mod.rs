use axum::{
    http::{StatusCode},
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use time::{OffsetDateTime};

async fn get_conn() -> Result<Pool<PostgresConnectionManager<NoTls>>, String> {

    // set up connection pool
    let manager =
        PostgresConnectionManager::new_from_stringlike("host=localhost user=postgres password=example", NoTls)
            .unwrap();
    Pool::builder().build(manager).await.map_err(|e| e.to_string())

    // // build our application with some routes
    // let app = Router::with_state(pool).route(
    //     "/",
    //     get(using_connection_pool_extractor).post(using_connection_extractor),
    // );
}

// type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

// async fn using_connection_pool_extractor(
//     State(pool): State<ConnectionPool>,
// ) -> Result<String, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//
//     let row = conn
//         .query_one("select 1 + 1", &[])
//         .await
//         .map_err(internal_error)?;
//     let two: i32 = row.try_get(0).map_err(internal_error)?;
//
//     Ok(two.to_string())
// }

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Debug)]
struct Visitor {
    name: String,
    mobile_phone_no: String,
    last_visit_date: OffsetDateTime,
}

async fn get_one_visitor(
    conn: &PooledConnection<'_, PostgresConnectionManager<NoTls>>,
) -> Result<Visitor, (StatusCode, String)> {
    let row = conn
        .query_one("select name, mobile_phone_no, last_visit_at from visitors", &[])
        .await
        .map_err(internal_error)?;
    // let x: Timestamp<Instant> = row.try_get(2).map_err(internal_error)?;
    let r: Visitor = Visitor {
        name: row.try_get(0).map_err(internal_error)?,
        mobile_phone_no: row.try_get(1).map_err(internal_error)?,
        last_visit_date: row.try_get(2).map_err(internal_error)?,
    };

    Ok(r)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_add() {
        let pool = get_conn().await.unwrap();
        let conn = pool.get().await.map_err(internal_error).unwrap();
        println!("{:?} ", get_one_visitor(&conn).await.unwrap());
        assert_eq!("aaaa", get_one_visitor(&conn).await.unwrap().name);
    }
}
