pub mod api;
pub mod config;
pub mod handlers;
pub mod routes;
pub mod view;

extern crate dotenv;

use crate::config::Config;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize)]
pub struct InstallQueryParams {
    hmac: String,
    shop: String,
    timestamp: String,
}

pub fn with_config(config: Arc<Config>) -> warp::filters::BoxedFilter<(Arc<Config>,)> {
    warp::any().map(move || config.clone()).boxed()
}
