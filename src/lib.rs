pub mod api;
pub mod config;
pub mod db_conn;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::{config::Config, db_conn::DbConn};
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use warp::Filter;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginFormBody {
    pub login_challenge: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginQueryParams {
    pub login_challenge: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallQueryParams {
    hmac: String,
    shop: String,
    timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfirmQueryParams {
    code: String,
    hmac: String,
    host: String,
    timestamp: String,
    state: String,
    shop: String,
}

pub fn with_config(config: Arc<Config>) -> warp::filters::BoxedFilter<(Arc<Config>,)> {
    warp::any().map(move || config.clone()).boxed()
}

pub fn with_db_conn(conn: Arc<DbConn>) -> warp::filters::BoxedFilter<(Arc<DbConn>,)> {
    warp::any().map(move || conn.clone()).boxed()
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_test() -> PgConnection {
    let database_url = db_test_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn db_test_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set")
}
