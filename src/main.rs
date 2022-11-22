mod apis;
mod data;

use std::borrow::Borrow;
use std::env;
use std::sync::Arc;
use axum::{
    routing::{get},
    Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use toml;
use serde_derive::Deserialize;
use tokio_postgres::NoTls;
use crate::apis::adm_visitors::{adm_visitors};
use crate::apis::new_visitor::{new_visitor_get, new_visitor_post};
use crate::data::get_pool;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct Config {
    pg_dsn: String,
}

#[tokio::main]
async fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("visitorreg requires a config file");
    //     return;
    // }
    //
    // // The first argument is the path that was used to call the program.
    // println!("My path is {}.", args[0]);
    // let conf: Config = toml::from_str(args[1].as_str()).unwrap();
    let conf = Config {
        pg_dsn: "host=localhost user=postgres password=example".to_string()
    };
    let p = get_pool(&conf.pg_dsn).await.unwrap();
    // let pool: &'static Pool<PostgresConnectionManager<NoTls>> = get_pool(&conf.pg_dsn).await.unwrap().borrow();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/adm/visitors", get(adm_visitors))
        .route("/new-visitor", get(new_visitor_get).post(new_visitor_post))
        .with_state(p);


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
