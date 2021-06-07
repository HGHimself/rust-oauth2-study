use crate::{config::Config, with_config, InstallQueryParams};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("shopify_install").boxed()
}

pub fn shopify_install(config: Arc<Config>) -> BoxedFilter<(InstallQueryParams, Arc<Config>)> {
    warp::get()
        .and(path_prefix())
        .and(warp::query::query::<InstallQueryParams>())
        .and(with_config(config))
        .boxed()
}

pub fn shopify_confirm(
    config: Arc<Config>,
) -> BoxedFilter<(Arc<Config>, String, String, String, String, String, String)> {
    warp::get()
        .and(path_prefix())
        .and(with_config(config))
        .and(warp::path::param::<String>()) // code
        .and(warp::path::param::<String>()) // hmac
        .and(warp::path::param::<String>()) // host
        .and(warp::path::param::<String>()) // timestamp
        .and(warp::path::param::<String>()) // state
        .and(warp::path::param::<String>()) // shop
        .boxed()
}
