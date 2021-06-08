pub mod api;
pub mod config;
pub mod handlers;
pub mod routes;

extern crate dotenv;

use crate::config::Config;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::env;
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

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_test() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
