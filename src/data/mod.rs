pub mod visitor;

use axum::{
    http::{StatusCode},
};
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub async fn get_conn() -> Result<Pool<PostgresConnectionManager<NoTls>>, String> {

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

