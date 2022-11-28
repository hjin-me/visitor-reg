mod apis;
mod data;

use std::{fs};
use axum::{
    routing::{get},
    Router,
};
use toml;
use serde_derive::Deserialize;
use tracing::{Level, trace, info};
use crate::apis::adm_visitors::{adm_visitors, export_visitors};
use crate::apis::AppState;
use crate::apis::login::{login_get, login_post, save_session_get};
use crate::apis::new_visitor::{new_visitor_get, new_visitor_post};
use crate::data::get_pool;
use clap::Parser;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct Config {
    pg_dsn: String,
    session_secret: String,
    default_uid: String,
    default_password: String,
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    info!("Starting up {}", &args.config);

    let contents = fs::read_to_string(&args.config)
        .expect("Should have been able to read the file");
    let conf: Config = toml::from_str(contents.as_str()).unwrap();
    // let conf = Config {
    //     pg_dsn: "host=localhost user=postgres password=example".to_string()
    // };
    trace!("pg_dsn: {}", conf.pg_dsn);
    let p = get_pool(&conf.pg_dsn).await.unwrap();
    // let pool: &'static Pool<PostgresConnectionManager<NoTls>> = get_pool(&conf.pg_dsn).await.unwrap().borrow();
    let app_state = AppState {
        pool: p,
        session_secret: conf.session_secret,

        allowed_uid: conf.default_uid,
        allowed_password: conf.default_password,
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/adm/visitors", get(adm_visitors))
        .route("/new-visitor", get(new_visitor_get).post(new_visitor_post))
        .route("/adm/in", get(login_get).post(login_post))
        .route("/adm/exchange", get(save_session_get))
        .route("/adm/export", get(export_visitors))
        .with_state(app_state);


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
