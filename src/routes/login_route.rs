use crate::view;
use crate::{LoginFormBody, LoginQueryParams};
use ory_hydra_client::apis::configuration::Configuration;
use std::sync::Arc;
use tera::{Context, Tera};
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("login").boxed()
}

pub fn login() -> BoxedFilter<(LoginQueryParams, Tera, Arc<Configuration>)> {
    warp::get()
        .and(path_prefix())
        .and(warp::query::query())
        .and(view::with_tera())
        .and(with_hydra_api())
        .boxed()
}

pub fn accept_login() -> BoxedFilter<(LoginFormBody, Arc<Configuration>)> {
    warp::post()
        .and(path_prefix())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and(with_hydra_api())
        .boxed()
}

pub fn with_hydra_api() -> warp::filters::BoxedFilter<(Arc<Configuration>,)> {
    warp::any()
        .map(move || {
            let mut configuration = Configuration::new();
            configuration.base_path = "localhost:4444".to_owned();
            Arc::new(configuration)
        })
        .boxed()
}
